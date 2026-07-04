// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/core/expr/identifiers.rs
//  Desc:       Identifier resolution and variable handling.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    types::{
        Instruction,
        opcode,
    },
    native::registry::is_native_function,
};
use crate::{
    shared::{
        types::{
            TokenType,
            Token,
        },
        utils::{
            Counter,
            Operators,
        },
    },
    compiler::parser::{
        errors::{
            ParseError,
            ParseErrorType,
        },
        types::{
            ParserContext,
            FuncFrame,
        },
        core::Parser,
    }
};

use TokenType as TT;

impl Parser {
    pub fn load_variable(ctx: &mut ParserContext, identifier: &str, token: &Token) -> Result<(), ParseError> {
        if let Some((access_opcode, var_idx)) = ctx.value_pool.get_var_access_opcode(identifier) {
            ctx.emit(Instruction::new_raw(access_opcode, var_idx).0, Some(token));
        } else if let Some(global_idx) = ctx.value_pool.get_bootstrap_global_index_by_name(identifier) {
            ctx.emit(Instruction::new_raw(opcode::GET_GLOBAL, global_idx).0, Some(token));
        } else if let Some(native_id) = is_native_function(identifier) {
            let const_idx = ctx.value_pool.get_or_create_native_fn(native_id);
            ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, const_idx).0, Some(token));
        } else {
            let suggestion = ctx.value_pool.suggest_variable_name(identifier);

            return Err(ParseError::from_token(
                ParseErrorType::UndefinedVariable { 
                    name: identifier.to_string(),
                    suggestion 
                },
                token
            ));
        }
        Ok(())
    }

    pub fn store_variable(ctx: &mut ParserContext, token: &Token) -> Result<(), ParseError> {
        let Some(name) = token.value.as_identifier() else {
            return Err(ParseError::from_token(ParseErrorType::InvalidAssignmentTarget, token));
        };

        ctx.value_pool.get_or_create_var_name(&name);

        if let Some((assign_opcode, var_idx)) = ctx.value_pool.get_var_assign_opcode(&name) {
            ctx.emit(Instruction::new_raw(assign_opcode, var_idx).0, Some(token));
        } else if let Some(global_idx) = ctx.value_pool.get_bootstrap_global_index_by_name(&name) {
            ctx.emit(Instruction::new_raw(opcode::SET_GLOBAL, global_idx).0, Some(token));
        } else {
            return Err(ParseError::from_token(ParseErrorType::FunctionResolutionError("Failed to resolve variable scope after creation"), token));
        }
        Ok(())
    }

    pub fn resolve_prototype(ctx: &mut ParserContext, identifier: &str, token: &Token) -> Result<(), ParseError> {
        if let Some((access_opcode, var_idx)) = ctx.value_pool.get_var_access_opcode(identifier) {
            ctx.emit(Instruction::new_raw(access_opcode, var_idx).0, Some(token));
        } else if let Some(global_idx) = ctx.value_pool.get_bootstrap_global_index_by_name(identifier) {
            ctx.emit(Instruction::new_raw(opcode::GET_GLOBAL, global_idx).0, Some(token));
        } else {
            let suggestion = ctx.value_pool.suggest_variable_name(identifier);

            return Err(ParseError::from_token(
                ParseErrorType::UndefinedVariable { 
                    name: identifier.to_string(),
                    suggestion 
                },
                token
            ));
        }
        Ok(())
    }

    pub fn resolve_identifier(
        token: &Token,
        tokens: &[Token],
        i: usize,
        depth: u32,
        main: bool,
        counter: &mut Counter,
        ctx: &mut ParserContext
    ) -> Result<(bool, bool), ParseError> {
        let identifier = token.value.as_identifier().unwrap_or("".to_string());
        
        let mut is_call = false;
        let is_dollar_start = matches!(token.typ, TokenType::DollarCall);
        let is_object_access = matches!(token.typ, TT::DotAccess | TT::NativeAccess);

        let is_dollar = is_dollar_start
            || (!ctx.func_frames.is_empty()
                && ctx.func_frames.last().unwrap().is_dollar
                && counter.is_zero());
        
        // Look at next token to determine context
        let next = tokens.get(i + 1);
        let next_is_assign = next.map_or(false, |t| Operators::is_assign_token(t.typ));

        if is_dollar_start {
            if next.map_or(false, |t| matches!(t.typ, TT::Op(_))) {
                return Err(ParseError::from_token(ParseErrorType::UnexpectedToken("unexpected operator in dollar call"), token));
            }
            if ctx.op_stack.is_empty() || ctx.op_stack.last().unwrap().typ != TT::LParen {
                return Err(ParseError::from_token(ParseErrorType::UnexpectedToken("unexpected dollar call"), token));
            }
        }

        if is_object_access {
            let const_idx = ctx.value_pool.get_or_create_string(&identifier, &mut ctx.memory_manager);
            ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, const_idx).0, Some(token));
        }

        if !next_is_assign {
            // Potential call context
            let is_explicit_call = next.map_or(false, |t| matches!(t.typ, TT::LParen));
            let is_implicit_call = !is_explicit_call 
                && (main || is_dollar || (ctx.func_frames.is_empty() && ctx.op_stack.is_empty()))
                && !next.map_or(false, |t| matches!(t.typ, TT::Op(_)))
                && !matches!(next.map(|t| t.typ).unwrap_or(TT::End), TT::LBracket | TT::LBrace | TT::DotAccess | TT::NativeAccess);
            
            is_call = is_explicit_call || is_implicit_call;
            
            if is_call {
                if is_dollar && is_object_access {
                    let f = ctx.func_frames.last_mut().unwrap();
                    f.is_method = true;
                    f.name = Some(identifier.to_string());
                } else {
                    ctx.func_frames.push(FuncFrame {
                        name: Some(identifier.to_string()),
                        depth: if is_explicit_call { depth + 1 } else { depth }, 
                        output_len: ctx.instructions.len() + 1, // +1 for the function reference
                        is_method: is_object_access, 
                        is_dollar: is_dollar,
                        is_explicit_call: is_explicit_call
                    });
                }

                if is_object_access {
                    if matches!(token.typ, TT::NativeAccess) {
                        // Native access
                        if is_call {
                            ctx.emit(Instruction::simple_raw(opcode::GET_NATIVE_METHOD).0, Some(token));
                        } else {
                            ctx.emit(Instruction::simple_raw(opcode::GET_NATIVE_MEMBER).0, Some(token));
                        }
                    } else {
                        // Regular access
                        if is_call {
                            ctx.emit(Instruction::simple_raw(opcode::GET_METHOD).0, Some(token));
                        } else {
                            ctx.emit(Instruction::simple_raw(opcode::GET_MEMBER).0, Some(token));
                        }
                    }
                } else {
                    Self::load_function(ctx, &identifier, token)?;
                }
                
                if !is_dollar && is_implicit_call {
                    counter.push(0);
                }
            } else {
                if is_object_access {
                    if matches!(token.typ, TT::NativeAccess) {
                        // Native access
                        ctx.emit(Instruction::simple_raw(opcode::GET_NATIVE_MEMBER).0, Some(token));
                    } else {
                        // Regular access
                        ctx.emit(Instruction::simple_raw(opcode::GET_MEMBER).0, Some(token));
                    }
                } else {
                    Self::load_variable(ctx, &identifier, token)?;
                }
                
                if is_dollar_start {
                    ctx.func_frames.push(FuncFrame {
                        name: Some(identifier.to_string()),
                        depth: depth, 
                        output_len: ctx.instructions.len(), 
                        is_method: false, 
                        is_dollar: true,
                        is_explicit_call: false
                    });
                }
            }
        }
        
        Ok((is_call, is_dollar))
    }
}
