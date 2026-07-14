// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/core/expr/operators.rs
//  Desc:       Operator processing and handling.
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
};
use crate::{
    shared::{
        types::{
            TokenType,
            Operator,
            Token,
            TokenValue,
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
            frames::LogicFrame,
        },
        core::Parser,
    },
};

use TokenType as TT;

impl Parser {
    fn get_bin_opcode(op: Operator) -> Option<u8> {
        match op {
            // Arithmetic
            Operator::AddAssign | Operator::Add => Some(opcode::ADD),
            Operator::SubAssign | Operator::Sub => Some(opcode::SUB),
            Operator::MulAssign | Operator::Mul => Some(opcode::MUL),
            Operator::DivAssign | Operator::Div => Some(opcode::DIV),
            Operator::FloorDivAssign | Operator::FloorDiv => Some(opcode::FLOOR_DIV),
            Operator::Mod => Some(opcode::MOD),
            Operator::Pow => Some(opcode::POW),
            Operator::Inc => Some(opcode::ADD),
            Operator::Dec => Some(opcode::SUB),
            
            // Comparison
            Operator::Eq => Some(opcode::EQ),
            Operator::Ne => Some(opcode::NE),
            Operator::Gt => Some(opcode::GT),
            Operator::Lt => Some(opcode::LT),
            Operator::Ge => Some(opcode::GE),
            Operator::Le => Some(opcode::LE),
            
            // Bitwise
            Operator::BitAnd => Some(opcode::BIT_AND),
            Operator::BitOr => Some(opcode::BIT_OR),
            Operator::BitXor => Some(opcode::BIT_XOR),
            
            // Other
            Operator::ConcatAssign | Operator::Dot => Some(opcode::DOT),
            Operator::Arrow => Some(opcode::INPUT),
            
            _ => None,
        }
    }

    pub fn process_op(token: Token, ctx: &mut ParserContext) -> Result<bool, ParseError> {
        match token.typ {
            TT::Op(op) => {
                match op {
                    Operator::Assign => {
                        Self::store_variable(ctx, &token)?;
                    },
                    Operator::Inc | Operator::Dec => {
                        // Postfix increment/decrement is syntactic sugar for i += 1

                        let is_member = matches!(token.value, TokenValue::Integer(1)) || matches!(token.value, TokenValue::Bool(true));
                        let is_index = matches!(token.value, TokenValue::Integer(2));

                        let Some(bin_op) = Self::get_bin_opcode(op) else {
                            return Err(ParseError::from_token(ParseErrorType::InvalidAssignmentTarget, &token));
                        };

                        if is_member || is_index {
                            let set_opcode = if is_index { opcode::SET_INDEX_PREP } else { opcode::SET_MEMBER_PREP };
                            ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, 1).0, Some(&token));
                            ctx.emit(Instruction::simple_raw(bin_op).0, Some(&token));
                            ctx.emit(Instruction::simple_raw(set_opcode).0, Some(&token));
                        } else {
                            // GET was already emitted by resolve_assign
                            ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, 1).0, Some(&token));
                            ctx.emit(Instruction::simple_raw(bin_op).0, Some(&token));
                            Self::store_variable(ctx, &token)?;
                        }
                    },
                    Operator::AddAssign | Operator::SubAssign | Operator::MulAssign | Operator::DivAssign | Operator::FloorDivAssign | Operator::ConcatAssign | Operator::Arrow => {
                        let is_member = matches!(token.value, TokenValue::Integer(1)) || matches!(token.value, TokenValue::Bool(true));
                        let is_index = matches!(token.value, TokenValue::Integer(2));
                        
                        let Some(bin_op) = Self::get_bin_opcode(op) else {
                            return Err(ParseError::from_token(ParseErrorType::InvalidAssignmentTarget, &token));
                        };
                        
                        if is_member || is_index {
                            let set_opcode = if is_index { opcode::SET_INDEX_PREP } else { opcode::SET_MEMBER_PREP };
                            ctx.emit(Instruction::simple_raw(bin_op).0, Some(&token));
                            ctx.emit(Instruction::simple_raw(set_opcode).0, Some(&token));
                        } else {
                            // GET was already emitted by resolve_assign
                            ctx.emit(Instruction::simple_raw(bin_op).0, Some(&token));
                            Self::store_variable(ctx, &token)?;
                        }
                    },
                    Operator::SetMember => {
                        ctx.emit(Instruction::simple_raw(opcode::SET_MEMBER).0, Some(&token));
                    },
                    Operator::SetIndex => {
                        ctx.emit(Instruction::simple_raw(opcode::SET_INDEX).0, Some(&token));
                    },
                    Operator::As => {
                        if let TokenValue::Integer(type_index) = token.value {
                            ctx.emit(Instruction::new_raw(opcode::TO, type_index as u32).0, Some(&token));
                        } else {
                            return Err(ParseError::from_token(ParseErrorType::InvalidOperator("invalid type index for 'as' operator"), &token));
                        }
                    },
                    Operator::And | Operator::Or => {
                        if let Some(frame) = ctx.logic_frames.last_mut() {
                            frame.count -= 1;
                            if frame.count == 0 {
                                let target = ctx.instructions.len() as u32;
                                for jump_idx in &frame.jumps {
                                    ctx.instructions[*jump_idx] = Instruction(ctx.instructions[*jump_idx]).with_payload(target).0;
                                }
                                ctx.logic_frames.pop();
                            }
                        }
                    },
                    Operator::Range => {
                        ctx.emit(Instruction::new_raw(opcode::MAKE_RANGE, 0).0, Some(&token));
                    },
                    Operator::RangeInclusive => {
                        ctx.emit(Instruction::new_raw(opcode::MAKE_RANGE, 1).0, Some(&token));
                    },
                    
                    _ => {
                        let opcode = if token.is_unary {
                            match op {
                                Operator::Sub => opcode::NEG,
                                Operator::Not => opcode::NOT,
                                _ => return Err(ParseError::from_token(
                                    ParseErrorType::InvalidOperator("invalid unary operator"), 
                                    &token
                                )),
                            }
                        } else {
                            Self::get_bin_opcode(op).ok_or_else(|| ParseError::from_token(
                                ParseErrorType::InvalidOperator("invalid binary operator"), 
                                &token
                            ))?
                        };
                        
                        ctx.emit(Instruction::simple_raw(opcode).0, Some(&token));
                    }
                }
            }

            TT::IsMatch | TT::InMatch => {
                let op = if token.typ == TT::IsMatch { opcode::EQ } else { opcode::IN };
                ctx.emit(Instruction::new_raw(opcode::DUP, 0).0, Some(&token));
                ctx.emit(Instruction::simple_raw(op).0, Some(&token));
                let jump_idx = ctx.instructions.len();
                ctx.emit(Instruction::new_raw(opcode::JUMP_IF, 0).0, Some(&token));
                if let Some(f) = ctx.cond_frames.last_mut() {
                    f.if_jumps.push(jump_idx);
                }
                ctx.op_stack.push(token.clone());   // Duplicate it for the next possible matching case
                return Ok(true);
            }

            _ => {
                let error_type = Parser::get_delimiter_error_message(token.typ);
                return Err(ParseError::from_token(error_type, &token));
            }
        }
        Ok(false)
    }

    pub fn handle_as_operator(
        token: &Token,
        tokens: &[Token],
        ctx: &mut ParserContext,
        advance: &mut usize,
    ) -> Result<(), ParseError> {

        if token.is_unary {
            return Err(ParseError::from_token(ParseErrorType::UnexpectedToken("unexpected 'is' operator"), token));
        }

        let next_token = tokens.get(*advance + 1).ok_or_else(|| {
            ParseError::from_token(ParseErrorType::ExpectedIdentifier("type identifier after 'as'"), token)
        })?;

        if !matches!(next_token.typ, TT::Identifier) {
            return Err(ParseError::from_token(ParseErrorType::ExpectedIdentifier("type identifier after 'as'"), next_token));
        }

        let type_identifier = next_token.value.as_identifier().ok_or_else(|| ParseError::from_token(
            ParseErrorType::MissingTokenValue,
            next_token
        ))?;
        
        match ctx.type_resolver.resolve_type(&type_identifier) {
            Some(type_index) => {
                let mut as_token = token.clone();
                as_token.value = TokenValue::Integer(type_index as i64);
                
                ctx.op_stack.push(as_token);

                *advance += 1;
            }
            None => {
                let suggestion = ctx.type_resolver.suggest_type(&type_identifier);
                
                return Err(ParseError::from_token(
                    ParseErrorType::UnknownType { 
                        name: type_identifier.clone(),
                        suggestion 
                    },
                    next_token
                ));
            }
        }
        Ok(())
    }

    fn handle_logic_operator(
        token: &Token,
        depth: u32,
        ctx: &mut ParserContext,
        jump_opcode: u8,
    ) -> Result<(), ParseError> {
        if !ctx.is_logic_depth(depth) {
            ctx.logic_frames.push(LogicFrame {
                depth,
                jumps: Vec::new(),
                count: 1,
            });
        } else if let Some(frame) = ctx.logic_frames.last_mut() {
            frame.count += 1;
        }

        ctx.emit(Instruction::new_raw(opcode::DUP, 0).0, Some(token));

        let jump_idx = ctx.instructions.len();
        ctx.emit(Instruction::new_raw(jump_opcode, 0).0, Some(token));
        if let Some(frame) = ctx.logic_frames.last_mut() {
            frame.jumps.push(jump_idx);
        }

        ctx.emit(Instruction::simple_raw(opcode::POP).0, Some(token));

        ctx.op_stack.push(token.clone());

        Ok(())
    }

    pub fn handle_and_operator(
        token: &Token,
        depth: u32,
        ctx: &mut ParserContext,
    ) -> Result<(), ParseError> {
        Self::handle_logic_operator(token, depth, ctx, opcode::JUMP_IF_NOT)
    }

    pub fn handle_or_operator(
        token: &Token,
        depth: u32,
        ctx: &mut ParserContext,
    ) -> Result<(), ParseError> {
        Self::handle_logic_operator(token, depth, ctx, opcode::JUMP_IF)
    }

    pub fn resolve_assign(
        op: Operator,
        token: &Token,
        tokens: &[Token],
        i: usize,
        ctx: &mut ParserContext,
        _source: &str,
    ) -> Result<(), ParseError> {
        let mut op_token = token.clone();
        
        if token.is_unary && (op == Operator::Inc || op == Operator::Dec) {
            return Err(ParseError::from_token(
                ParseErrorType::PrefixIncrementDecrementNotSupported,
                token
            ));
        }
        
        if Operators::is_assign(op) && !token.is_unary {
            if i > 0 {
                if let Some(prev) = tokens.get(i-1) {
                    if matches!(prev.typ, TT::DotAccess | TT::RBracket) {
                        // Assignment to object property or array index
                        if op == Operator::Assign {
                            op_token.typ = TT::Op(if prev.typ == TT::RBracket { Operator::SetIndex } else { Operator::SetMember });
                        } else {
                            op_token.value = TokenValue::Integer(if prev.typ == TT::RBracket { 2 } else { 1 });
                            if prev.typ == TT::RBracket {
                                ctx.emit(Instruction::simple_raw(opcode::GET_INDEX_PREP).0, Some(&prev));
                            } else {
                                ctx.emit(Instruction::simple_raw(opcode::GET_MEMBER_PREP).0, Some(&prev));
                            }
                        }
                    } else if matches!(prev.typ, TT::NativeAccess) {
                        // Native properties are read-only
                        return Err(ParseError::from_token(
                            ParseErrorType::NativePropertyAssignment,
                            &token
                        ));
                    } else if matches!(prev.typ, TT::Identifier) {
                        // Assignment to variable
                        op_token.value = prev.value.clone();
                        if op != Operator::Assign {
                            if let Some(name) = prev.value.as_identifier() {
                                if op == Operator::Arrow {
                                    ctx.value_pool.get_or_create_var_name(&name);
                                }
                                Self::load_variable(ctx, &name, &prev)?;
                            }
                        }
                    }
                }
            }
        }
        ctx.op_stack.push(op_token);
        Ok(())
    }

    pub fn resolve_operator(
        op: Operator,
        token: &Token,
        tokens: &[Token],
        i: &mut usize,
        depth: u32,
        counter: &mut Counter,
        ctx: &mut ParserContext,
        source: &str,
    ) -> Result<(), ParseError> {

        if op == Operator::Colon
        && ctx.is_object_depth(depth)
        && counter.is_even()
        {
            Self::pop_to_any_delim(ctx)?;
            counter.add();
            return Ok(());
        }

        while let Some(top) = ctx.op_stack.last() {
            if let TT::Op(top_op) = &top.typ {
                let p1 = Operators::precedence(op, token.is_unary);
                let p2 = Operators::precedence(*top_op, top.is_unary);
                if (!token.is_unary && p1 > p2) || (token.is_unary && p1 >= p2) {
                    break;
                }
                Self::process_op(ctx.op_stack.pop().unwrap(), ctx)?;
            } else {
                break;
            }
        }

        if op == Operator::As {
            Self::handle_as_operator(token, tokens, ctx, i)?;
            return Ok(());
        }

        if op == Operator::And {
            Self::handle_and_operator(token, depth, ctx)?;
            return Ok(());
        }

        if op == Operator::Or {
            Self::handle_or_operator(token, depth, ctx)?;
            return Ok(());
        }

        if op == Operator::Backdot {
            return Err(ParseError::from_token(
                ParseErrorType::UnsupportedBackdotOperator,
                token
            ));
        }

        Self::resolve_assign(op, token, tokens, *i, ctx, source)?;

        Ok(())
    }
}
