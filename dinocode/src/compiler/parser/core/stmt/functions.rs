// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/core/stmt/functions.rs
//  Desc:       Function parsing and loading utilities.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    types::{
        Instruction,
        DinoRef,
        opcode,
        UserFunction,
    },
    native::registry::is_native_function,
};
use crate::{
    shared::types::Token,
    compiler::parser::{
        errors::{
            ParseError,
            ParseErrorType,
        },
        types::ParserContext,
        core::Parser,
    },
};

impl Parser {
    pub fn load_function(ctx: &mut ParserContext, name: &str, token: &Token) -> Result<(), ParseError> {
        if let Some((scope_level, _var_idx)) = ctx.value_pool.resolve_var_scope_with_level(name) {
            let current_scope_depth = ctx.value_pool.get_current_scope_depth();
            
            if scope_level == 0 || scope_level == current_scope_depth {
                if let Some((access_opcode, var_idx)) = ctx.value_pool.get_var_access_opcode(name) {
                    ctx.emit(Instruction::new_raw(access_opcode, var_idx).0, Some(token));
                } else {
                    return Err(ParseError::from_token(ParseErrorType::FunctionResolutionError("function not found in this scope"), token));
                }
            } else {
                return Err(ParseError::from_token(ParseErrorType::FunctionResolutionError("cannot access function from a parent scope"), token));
            }
        } else if let Some(native_id) = is_native_function(name) {
            let const_idx = ctx.value_pool.get_or_create_native_fn(native_id);
            ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, const_idx).0, Some(token));
        } else if let Some(global_idx) = ctx.value_pool.get_bootstrap_global_index_by_name(name) {
            ctx.emit(Instruction::new_raw(opcode::GET_GLOBAL, global_idx).0, Some(token));
        } else {
            return Err(ParseError::from_token(ParseErrorType::FunctionResolutionError("function not found"), token));
        }
        Ok(())
    }
    
    pub fn create_function_call(ctx: &mut ParserContext, args: usize, is_method: bool) -> Result<(), ParseError> {
        // Check if we're in global scope and making a function call
        // If so, disable main registration since this indicates global code execution
        let current_scope_depth = ctx.value_pool.get_current_scope_depth();
        if current_scope_depth == 0 {
            ctx.allow_main = false;
        }
        
        let final_args = if is_method { args + 1 } else { args };
        ctx.emit(Instruction::new_raw(opcode::CALL, final_args as u32).0, None);
        Ok(())
    }
    
    pub fn handle_return_statement(ctx: &mut ParserContext, token: &Token, args: usize) -> Result<(), ParseError> {
        if args > 1 {
            return Err(ParseError::from_token(
                ParseErrorType::MultipleReturnValues,
                token
            ));
        }
        
        if ctx.function_def_frames.is_empty() {
            return Err(ParseError::from_token(
                ParseErrorType::InvalidControlFlow("return statement outside function"),
                token
            ));
        }
                
        if args > 0 {
            ctx.emit(Instruction::simple_raw(opcode::RETURN_REF).0, Some(token));
        } else {
            ctx.emit(Instruction::simple_raw(opcode::RETURN).0, Some(token));
        }
        
        Ok(())
    }

    pub fn register_main_if_allowed(ctx: &mut ParserContext) -> Result<bool, ParseError> {
        if !ctx.allow_main {
            return Ok(false);
        }
        
        let main_functions: Vec<(u32, &UserFunction)> = ctx.value_pool.get_functions().iter()
            .enumerate()
            .filter(|(_, func)| func.is_main)
            .map(|(index, func)| (index as u32, func))
            .collect();
        
        if main_functions.len() > 1 {
            return Err(ParseError::new(ParseErrorType::MultipleMainFunction, 1, 1));
        }
        
        if let Some((function_id, _function)) = main_functions.first() {
            let main_dinoref = DinoRef::function(*function_id);
            ctx.main_function = Some(main_dinoref);
            return Ok(true);
        }
        
        Ok(false)
    }
}
