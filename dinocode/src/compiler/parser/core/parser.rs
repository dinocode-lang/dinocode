// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/core/parser.rs
//  Desc:       Handles parsing of tokens into bytecode.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    types::{
        opcode,
        Instruction,
        Symbol,
    },
    utils::source_map::SourceMap,
};
use crate::{
    shared::{
        types::{
            TokenType,
            Token,
            TokenValue,
        },
        errors::{
            ParseError,
            ParseErrorType,
            pretty_parse_error,
        },
        utils::{
            Counter,
            Operators,
        },
    },
    compiler::parser::{
        types::{
            ParserContext,
            Bytecode,
            frames::{
                CondFrame,
                ArrayFrame,
                ObjectFrame,
                FunctionDefFrame,
                FuncFrame,
                ClassFrame,
                GroupFrame,
            },
        },
    },
};

use TokenType as TT;

pub struct Parser;

impl Parser {
    pub const STACK_DELIMITERS: [TokenType; 18] = [TT::LParen, TT::LBracket, TT::LBrace, TT::LString, TT::LBraceExpr, TT::Indent, TT::End, TT::If, TT::Else, TT::Elif, TT::While, TT::For, TT::Return, TT::Break, TT::Continue, TT::Class, TT::Is, TT::In];

    pub fn compile(tokens: &[Token], source: &str) -> Result<(Bytecode, SourceMap), ParseError> {
        let mut counter = Counter::new();
        let mut depth: u32 = 0;
        let mut main = true;
        let mut i = 0;
        let tokens_len = tokens.len();

        let none_inst = Instruction::simple_raw(opcode::NONE).0;
        let true_inst = Instruction::simple_raw(opcode::TRUE).0;
        let false_inst = Instruction::simple_raw(opcode::FALSE).0;
        let empty_str_inst = Instruction::new_raw(opcode::LOAD_CONST, 0).0;
        
        let mut ctx = ParserContext::new(tokens_len, source);

        while i < tokens_len {
            let token = &tokens[i];
            let typ = token.typ;

            match typ {
                TT::Integer | TT::BigInt | TT::Float | TT::Bool | TT::String => {
                    match &token.value {
                        TokenValue::Integer(i) => {
                            let const_idx = ctx.value_pool.get_or_create_int(*i, &mut ctx.memory_manager);
                            ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, const_idx).0, Some(token));
                        }
                        TokenValue::Float(f) => {
                            let const_idx = ctx.value_pool.get_or_create_float(*f, &mut ctx.memory_manager);
                            ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, const_idx).0, Some(token));
                        }
                        TokenValue::BigInt(bi) => {
                            let const_idx = match ctx.value_pool.get_or_create_bigint(bi, &mut ctx.memory_manager) {
                                Ok(idx) => idx,
                                Err(err_msg) => return Err(pretty_parse_error(ParseErrorType::Custom(err_msg), token.line.unwrap_or(1) as usize, token.column.unwrap_or(1) as usize, source)),
                            };
                            ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, const_idx).0, Some(token));
                        }
                        TokenValue::Bool(b) => {
                            if *b {
                                ctx.emit(true_inst, Some(token));
                            } else {
                                ctx.emit(false_inst, Some(token));
                            }
                        }
                        TokenValue::String(s) => {
                            let const_idx = ctx.value_pool.get_or_create_string(s, &mut ctx.memory_manager);
                            ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, const_idx).0, Some(token));
                        }
                        TokenValue::None => {
                            ctx.emit(none_inst, Some(token));
                        }
                    }
                }

                TT::Identifier => {
                    if ctx.is_object_depth(depth) && counter.is_even() {
                        let identifier = token.value.as_identifier().unwrap_or("".to_string());
                        let const_idx = ctx.value_pool.get_or_create_string(&identifier, &mut ctx.memory_manager);
                        ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, const_idx).0, Some(token));
                    } else {
                        Self::resolve_identifier(token, tokens, i, depth, main, &mut counter, &mut ctx)?;
                    }
                }

                TT::DollarCall => {
                    Self::resolve_identifier(token, tokens, i, depth, main, &mut counter, &mut ctx)?;
                }

                TT::DotAccess => {
                    Self::resolve_identifier(token, tokens, i, depth, main, &mut counter, &mut ctx)?;
                }

                TT::NativeAccess => {
                    Self::resolve_identifier(token, tokens, i, depth, main, &mut counter, &mut ctx)?;
                }

                TT::Comma => {
                    Self::pop_to_any_delim(&mut ctx)?;
                    counter.add();
                }

                TT::Op(op) => {
                    Self::resolve_operator(op, token, &tokens, &mut i, depth, &mut counter, &mut ctx, source)?;
                }

                TT::LParen => {
                    counter.push(0);
                    depth += 1;
                    ctx.op_stack.push(token.clone());
                    if !ctx.is_call_depth(depth) {
                        let is_dynamic_call = Self::is_continuous(i, &tokens);
                        if is_dynamic_call {
                            ctx.func_frames.push(FuncFrame {
                                name: None,
                                depth: depth, 
                                output_len: ctx.instructions.len(),
                                is_method: false, 
                                is_dollar: false,
                                is_explicit_call: true
                            });
                        } else {
                            ctx.group_frames.push(GroupFrame { depth, output_len: ctx.instructions.len() });
                        }
                    }
                }

                TT::RParen => {
                    let count = counter.pop();
                    Self::pop_until_delim(&mut ctx, TT::LParen, token)?;
                    if let Some(f) = ctx.call_with_depth(depth) {
                        let args = if !f.is_dollar && ctx.instructions.len() > f.output_len { count + 1 } else { count };
                        Self::create_function_call(&mut ctx, args, f.is_method)?;
                    } else if let Some(frame) = ctx.group_with_depth(depth) {
                        if ctx.instructions.len() == frame.output_len {
                            return Err(pretty_parse_error(ParseErrorType::ExpectedExpression("()".to_string()), token.line.unwrap_or(1) as usize, token.column.unwrap_or(1) as usize, ctx.source));
                        }
                    }

                    depth -= 1;
                }

                TT::LBraceExpr => {
                    counter.push(0);
                    depth += 1;
                    ctx.op_stack.push(token.clone());
                    ctx.group_frames.push(GroupFrame { depth, output_len: ctx.instructions.len() });
                }

                TT::RBraceExpr => {
                    counter.pop();
                    if let Some(frame) = ctx.group_with_depth(depth) {
                        if ctx.instructions.len() == frame.output_len {
                            return Err(pretty_parse_error(ParseErrorType::ExpectedExpression("${}".to_string()), token.line.unwrap_or(1) as usize, token.column.unwrap_or(1) as usize, ctx.source));
                        }
                    }
                    Self::pop_until_delim(&mut ctx, TT::LBraceExpr, token)?;
                    depth -= 1;
                }

                TT::LBracket => {
                    ctx.op_stack.push(token.clone());
                    counter.push(0);
                    depth += 1;
                    let create = !Self::is_continuous(i, &tokens);
                    ctx.array_frames.push(ArrayFrame { depth, output_len: ctx.instructions.len(), create });
                }

                TT::RBracket => {
                    Self::pop_until_delim(&mut ctx, TT::LBracket, token)?;
                    if let Some(a) = ctx.array_with_depth(depth) {
                        let count = counter.pop() + if ctx.instructions.len() > a.output_len { 1 } else { 0 };
                        if a.create {
                            ctx.emit(Instruction::new_raw(opcode::MAKE_ARRAY, count as u32).0, Some(token));
                        } else {
                            let next_is_assign = tokens.get(i + 1).map_or(false, |t| Operators::is_assign_token(t.typ));
                            if !next_is_assign {
                                ctx.emit(Instruction::simple_raw(opcode::GET_INDEX).0, Some(token));
                            }
                        }
                    }
                    depth -= 1;
                }
                
                TT::LBrace => {
                    ctx.op_stack.push(token.clone());
                    counter.push(0);
                    depth += 1;
                    let create = !Self::is_continuous(i, &tokens);
                    ctx.object_frames.push(ObjectFrame { depth, output_len: ctx.instructions.len(), create });
                }

                TT::RBrace => {
                    Self::pop_until_delim(&mut ctx, TT::LBrace, token)?;
                    if let Some(o) = ctx.object_with_depth(depth) {
                        let count = counter.pop() + if ctx.instructions.len() > o.output_len { 1 } else { 0 };
                        if o.create {
                            ctx.emit(Instruction::new_raw(opcode::MAKE_OBJECT, count as u32).0, Some(token));
                        }
                    }
                    depth -= 1;
                }

                TT::LString => {
                    let is_empty = tokens.get(i + 1).map_or(false, |t| t.typ == TT::RString);
                    if is_empty {
                        ctx.emit(empty_str_inst, Some(token));
                        i += 1;
                    } else {
                        ctx.op_stack.push(token.clone());
                        counter.push(0);
                        depth += 1;
                    }
                }
                TT::RString => {
                    Self::pop_until_delim(&mut ctx, TT::LString, token)?;
                    let count = counter.pop();
                    if count >= 1 {
                        ctx.emit(Instruction::new_raw(opcode::STR_BUILD, count as u32 + 1).0, Some(token));
                    }
                    depth -= 1;
                }

                TT::If | TT::Elif | TT::Else => {
                    ctx.op_stack.push(token.clone());
                    counter.push(0);
                }

                TT::Is | TT::In => {
                    let match_count = ctx.cond_frames.last()
                        .map_or(0, |f| f.expr_count);
                    if match_count > 0 {
                        let mut match_token = token.clone();
                        match_token.typ = if typ == TT::Is { TT::IsMatch } else { TT::InMatch };
                        ctx.op_stack.push(token.clone());
                        ctx.op_stack.push(match_token);
                        counter.push(
                            if tokens.get(i + 1).map_or(false, |token| token.typ == TokenType::Comma)
                            {
                                // The comma indicates a next argument,
                                // but in this particular case we need to prevent this comma from being processed
                                // in order to keep the virtual delimiter (IsMatch) still on the stack
                                i += 1;
                                1
                            }
                            else
                            {
                                0
                            }
                        );
                    } else {
                        let err_msg = if typ == TT::Is { "'Is' statement outside of conditional block" } else { "'In' statement outside of conditional block" };
                        return Err(pretty_parse_error(
                            ParseErrorType::Custom(err_msg.to_string()),
                            token.line.unwrap_or(0) as usize,
                            token.column.unwrap_or(0) as usize,
                            ctx.source
                        ));
                    }
                }

                TT::While => {
                    let loop_start = ctx.instructions.len();
                    ctx.cond_frames.push(CondFrame {
                        cond_jump: None,
                        exit_jumps: Vec::new(),
                        if_jumps: Vec::new(),
                        current_type: typ,
                        loop_start: Some(loop_start),
                        temp_vars_to_cleanup: Vec::new(),
                        expr_count: 0,
                        count: 0,
                        pop_at_end: false,
                    });
                    
                    ctx.op_stack.push(token.clone());
                }

                TT::For => {
                    let var_token = tokens.get(i + 1);
                    let var_name = match var_token {
                        Some(t) if matches!(t.typ, TT::Identifier) => t.value.as_identifier().unwrap_or_default(),
                        _ => return Err(pretty_parse_error(
                            ParseErrorType::Custom("Expected identifier after 'for'".to_string()),
                            var_token.and_then(|t| t.line).unwrap_or(token.line.unwrap_or(1)) as usize,
                            var_token.and_then(|t| t.column).unwrap_or(token.column.unwrap_or(1)) as usize,
                            source
                        )),
                    };

                    let in_token = tokens.get(i + 3);   // TT::Comma, TT::In
                    if !matches!(in_token, Some(t) if matches!(t.typ, TT::In)) {
                        return Err(pretty_parse_error(
                            ParseErrorType::Custom("Expected 'in' after variable in for loop".to_string()),
                            in_token.and_then(|t| t.line).unwrap_or(token.line.unwrap_or(1)) as usize,
                            in_token.and_then(|t| t.column).unwrap_or(token.column.unwrap_or(1)) as usize,
                            source
                        ));
                    }

                    ctx.value_pool.get_or_create_var_name(&var_name);
                    i += 3;
                    
                    ctx.cond_frames.push(CondFrame {
                        cond_jump: None,
                        exit_jumps: Vec::new(),
                        if_jumps: Vec::new(),
                        current_type: typ,
                        loop_start: None,
                        temp_vars_to_cleanup: Vec::new(),
                        expr_count: 0,
                        count: 0,
                        pop_at_end: false,
                    });
                    
                    let mut for_token = token.clone();
                    for_token.value = TokenValue::String(var_name);
                    ctx.op_stack.push(for_token);
                }

                TT::Return => {
                    ctx.op_stack.push(token.clone());
                    counter.push(0);
                }

                TT::Break => {
                    let loop_frame_idx = ctx.cond_frames.iter().rposition(|f| {
                        f.current_type == TT::While || f.current_type == TT::For
                    });

                    if loop_frame_idx.is_none() {
                        return Err(pretty_parse_error(
                            ParseErrorType::BreakOutsideLoop,
                            token.line.unwrap_or(0) as usize,
                            token.column.unwrap_or(0) as usize,
                            ctx.source
                        ));
                    }

                    let break_idx = ctx.instructions.len();
                    ctx.emit(Instruction::new_raw(opcode::JUMP, 0).0, Some(token));

                    if let Some(idx) = loop_frame_idx {
                        ctx.cond_frames[idx].exit_jumps.push(break_idx);
                    }

                    i += 2;
                    continue;
                }

                TT::Continue => {
                    let loop_frame = ctx.cond_frames.iter().rev().find(|f| {
                        (f.current_type == TT::While || f.current_type == TT::For) && f.loop_start.is_some()
                    });

                    if loop_frame.is_none() {
                        return Err(pretty_parse_error(
                            ParseErrorType::ContinueOutsideLoop,
                            token.line.unwrap_or(0) as usize,
                            token.column.unwrap_or(0) as usize,
                            ctx.source
                        ));
                    }

                    let target = loop_frame.unwrap().loop_start.unwrap();
                    ctx.emit(Instruction::new_raw(opcode::JUMP, target as u32).0, Some(token));
                    
                    i += 2;
                    continue;
                }

                TT::Class => {
                    let name_token = tokens.get(i + 1).ok_or_else(|| {
                        pretty_parse_error(ParseErrorType::ExpectedClassName, token.line.unwrap_or(1) as usize, token.column.unwrap_or(1) as usize, source)
                    })?;

                    let class_name = name_token.value.as_identifier().ok_or_else(|| {
                        pretty_parse_error(ParseErrorType::ExpectedClassName, name_token.line.unwrap_or(1) as usize, name_token.column.unwrap_or(1) as usize, source)
                    })?;

                    let parent_info = match (tokens.get(i + 2), tokens.get(i + 3)) {
                        (Some(comma_token), Some(parent_token)) 
                            if comma_token.typ == TT::Comma && parent_token.typ == TT::Identifier => {
                            if let Some(parent_name) = parent_token.value.as_identifier() {
                                Some((parent_name, parent_token, i + 3))
                            } else { None }
                        }
                        _ => None,
                    };

                    if let Some((parent_name, parent_token, next_i)) = parent_info {
                        Self::resolve_prototype(&mut ctx, &parent_name, &parent_token)?;
                        i = next_i;
                    } else {
                        ctx.emit(Instruction::simple_raw(opcode::NONE).0, Some(name_token));
                        i += 1;
                    }

                    ctx.class_frames.push(ClassFrame {
                        name: class_name,
                        start_ip: ctx.instructions.len(),
                        end_ip: 0,
                        initial_depth: depth,
                        method_count: 0,
                    });
                }

                TT::Function => {
                    let next_token = tokens.get(i + 1).ok_or_else(|| {
                        pretty_parse_error(ParseErrorType::ExpectedFunctionName, token.line.unwrap_or(1) as usize, token.column.unwrap_or(1) as usize, source)
                    })?;

                    if next_token.typ != TT::Identifier {
                        return Err(pretty_parse_error(ParseErrorType::ExpectedFunctionName, next_token.line.unwrap_or(1) as usize, next_token.column.unwrap_or(1) as usize, source));
                    }

                    let func_name = next_token.value.as_identifier().unwrap_or_default();
                    let is_main = func_name == "main";
                    let is_global = depth == 0;
                    
                    let function_id = ctx.value_pool.register_function_placeholder(is_main);
                    ctx.value_pool.get_or_create_function(function_id);
                    ctx.value_pool.get_or_create_var_name(&func_name);
                    ctx.value_pool.push_scope();

                    let is_method = ctx.class_frames.last()
                        .map_or(false, |frame| depth == frame.initial_depth + 1);

                    let mut param_count = if is_method {
                        ctx.value_pool.get_or_create_var_name("self");
                        1
                    } else {
                        0
                    };

                    let mut j = i + 2;
                    while let Some(t) = tokens.get(j) {
                        match t.typ {
                            TT::Comma => param_count += 1,
                            TT::Identifier => {
                                if let Some(id) = t.value.as_identifier() {
                                    ctx.value_pool.get_or_create_var_name(&id);
                                }
                            }
                            TT::Indent => break,
                            _ => return Err(pretty_parse_error(
                                ParseErrorType::UnexpectedTokenInParameterList,
                                t.line.unwrap_or(1) as usize,
                                t.column.unwrap_or(1) as usize,
                                source
                            )),
                        }
                        j += 1;
                    }

                    i = j - 1;
                    ctx.emit(Instruction::new_raw(opcode::JUMP, 0).0, Some(token));

                    ctx.function_def_frames.push(FunctionDefFrame {
                        name: func_name,
                        start_ip: ctx.instructions.len(),
                        end_ip: 0,
                        param_count,
                        return_count: 1,
                        initial_depth: depth,
                        function_id,
                        is_global,
                        is_method,
                    });
                }

                TT::Indent => {
                    main = true;
                    depth += 1;
                    Self::pop_to_delim(&mut ctx, |t| matches!(t, TT::If | TT::Elif | TT::Else | TT::While | TT::For | TT::Function | TT::Is | TT::IsMatch | TT::In | TT::InMatch))?;
                    if let Some(top) = ctx.op_stack.pop() {
                        let typ = top.typ;
                        match typ {
                            TT::If => {
                                let cond_idx = ctx.instructions.len();
                                let is_match = tokens.get(i + 1).map_or(false, |t| t.typ == TT::Is || t.typ == TT::In);
                                
                                if !is_match {
                                    ctx.emit(Instruction::new_raw(opcode::JUMP_IF_NOT, 0).0, Some(&top));
                                }
                                
                                ctx.cond_frames.push(CondFrame {
                                    cond_jump: if is_match { None } else { Some(cond_idx) },
                                    exit_jumps: Vec::new(),
                                    if_jumps: Vec::new(),
                                    current_type: typ,
                                    loop_start: None,
                                    temp_vars_to_cleanup: Vec::new(),
                                    expr_count: counter.get(),
                                    count: 0,
                                    pop_at_end: false,
                                });
                            }
                            TT::Elif | TT::Else => {
                                if let Some(frame) = ctx.cond_frames.last_mut() {
                                    frame.current_type = typ;
                                    if typ == TT::Elif {
                                        frame.cond_jump = Some(ctx.instructions.len());
                                        ctx.emit(Instruction::new_raw(opcode::JUMP_IF_NOT, 0).0, Some(&top));
                                    } else {
                                        frame.cond_jump = None;
                                    }
                                }
                            }

                            TT::While => {
                                let cond_idx = ctx.instructions.len();
                                ctx.emit(Instruction::new_raw(opcode::JUMP_IF_NOT, 0).0, Some(&top));
                                if let Some(frame) = ctx.cond_frames.last_mut() {
                                    frame.cond_jump = Some(cond_idx);
                                    frame.temp_vars_to_cleanup = Vec::new();
                                }
                            }
                            
                            TT::For => {
                                
                                // [target, index, limit, step]
                                let iter_var = ctx.value_pool.allocate_temp_var();
                                let _iter_idx = ctx.value_pool.allocate_temp_var();
                                let _iter_limit = ctx.value_pool.allocate_temp_var();
                                let _iter_step = ctx.value_pool.allocate_temp_var();
                                
                                if let Some((_assign_opcode, iter_var_idx)) = ctx.value_pool.get_var_assign_opcode(&iter_var) {
                                    ctx.emit(Instruction::new_raw(opcode::FOR_INIT, iter_var_idx).0, Some(&top));
                                    
                                    if let Some(frame) = ctx.cond_frames.last_mut() {
                                        frame.temp_vars_to_cleanup.push((opcode::FOR_DROP, iter_var_idx));
                                    }
                                }
                                
                                let loop_start = ctx.instructions.len();
                                
                                if let Some((_access_opcode, iter_var_idx)) = ctx.value_pool.get_var_access_opcode(&iter_var) {
                                    ctx.emit(Instruction::new_raw(opcode::FOR_ITER, iter_var_idx).0, Some(&top));
                                }
                                
                                let exit_jump_idx = ctx.instructions.len();
                                ctx.emit(Instruction::new_raw(opcode::JUMP_IF_NOT, 0).0, Some(&top));
                                
                                let var_name = top.value.as_str().unwrap_or("").to_string();
                                ctx.value_pool.get_or_create_var_name(&var_name);
                                
                                if let Some((assign_opcode, var_idx)) = ctx.value_pool.get_var_assign_opcode(&var_name) {
                                    ctx.emit(Instruction::new_raw(assign_opcode, var_idx).0, Some(&top));
                                    ctx.emit(Instruction::simple_raw(opcode::POP).0, Some(&top));
                                }
                                
                                if let Some(frame) = ctx.cond_frames.last_mut() {
                                    frame.loop_start = Some(loop_start);
                                    frame.cond_jump = Some(exit_jump_idx);
                                }
                            }
                            
                            TT::Is | TT::IsMatch | TT::In | TT::InMatch => {
                                if typ != TT::Is && typ != TT::In {
                                    /*
                                    * Shadowing of 'Is' by 'IsMatch'
                                    * ------------------------------------------------
                                    * The virtual delimiter 'IsMatch' duplicates itself for each matching case, 
                                    * meaning that in the previous pop_to_delim() it was auto-inserted.
                                    * 
                                    * It is necessary to manually discard the 'Is' delimiter.
                                    */
                                    ctx.op_stack.pop();
                                }

                                // Last Matching value
                                ctx.emit(Instruction::new_raw(opcode::DUP, 0).0, Some(&token));
                                let opc = if typ == TT::Is || typ == TT::IsMatch { opcode::EQ } else { opcode::IN };
                                ctx.emit(Instruction::simple_raw(opc).0, Some(&token));
                                
                                let mut jump_idx = ctx.instructions.len();
                                ctx.emit(Instruction::new_raw(opcode::JUMP_IF_NOT, 0).0, Some(&token));
                                
                                let count = counter.pop();
                                if count == 0 {
                                    return Err(pretty_parse_error(
                                        ParseErrorType::EmptyMatchComparison,
                                        top.line.unwrap_or(1) as usize,
                                        top.column.unwrap_or(1) as usize,
                                        ctx.source
                                    ));
                                }
                                if let Some(f) = ctx.cond_frames.last_mut() {
                                    f.cond_jump = Some(jump_idx);
                                    f.current_type = typ;
                                    f.count = 1;
                                    f.pop_at_end = true;
                                    let mut expr_count = f.expr_count;
                                    let is_single_match = if expr_count == 1 {
                                        true
                                    } else if expr_count == count {
                                        f.exit_jumps.extend(f.if_jumps.clone());
                                        false
                                    } else {
                                        return Err(pretty_parse_error(
                                            ParseErrorType::MatchCorrespondenceError { expected_values: expr_count, actual_values: count },
                                            top.line.unwrap_or(1) as usize,
                                            top.column.unwrap_or(1) as usize,
                                            ctx.source
                                        ));
                                    };

                                    f.if_jumps.push(jump_idx);

                                    expr_count += 1;    // REF_1 REF_2 DUP(2) EQ JUMP_IF*
                                    jump_idx += 1;      // After last JUMP_IF_NOT
                                    for if_idx in &f.if_jumps {
                                        let idx = *if_idx;
                                        let dup_idx = idx - 2;
                                        ctx.instructions[dup_idx] = Instruction(ctx.instructions[dup_idx]).with_payload((expr_count - 1) as u32).0;
                                        if is_single_match {
                                            ctx.instructions[idx] = Instruction(ctx.instructions[idx]).with_payload(jump_idx as u32).0;
                                        } else {
                                            ctx.instructions[idx] = Instruction::new_raw(opcode::JUMP_IF_NOT, 0).0;
                                            expr_count -= 1;
                                        }
                                    }
                                    f.if_jumps.clear();
                                }
                            }

                            _ => {}
                        }
                    }

                    i += 1;
                    continue
                }

                TT::Dedent => {
                    main = true;
                    depth -= 1;
                    
                    let (is_loop, loop_start, cond_jump, temp_vars_to_cleanup) = ctx.cond_frames.last()
                        .filter(|f| f.current_type == TT::While || f.current_type == TT::For)
                        .map(|f| (true, f.loop_start, f.cond_jump, f.temp_vars_to_cleanup.clone()))
                        .unwrap_or((false, None, None, Vec::new()));
                    
                    if is_loop {
                        if let Some(l_start) = loop_start {
                            ctx.emit(Instruction::new_raw(opcode::JUMP, l_start as u32).0, Some(token));
                        }
                        
                        // Patch exit jump
                        let target_idx = ctx.instructions.len();
                        if let Some(cond_idx) = cond_jump {
                            ctx.instructions[cond_idx] = Instruction(ctx.instructions[cond_idx]).with_payload(target_idx as u32).0;
                        }

                        // Generate cleanup code for temporary variables
                        if !temp_vars_to_cleanup.is_empty() {
                            for (drop_opcode, var_idx) in temp_vars_to_cleanup {
                                ctx.emit(Instruction::new_raw(drop_opcode, var_idx).0, Some(token));
                            }
                        }

                        // Patch breaks
                        if let Some(final_frame) = ctx.cond_frames.pop() {
                            for exit_idx in final_frame.exit_jumps {
                                ctx.instructions[exit_idx] = Instruction(ctx.instructions[exit_idx]).with_payload(target_idx as u32).0;
                            }
                        }

                        i += 1;
                        continue;
                    }

                    // Check if this is a function dedent
                    let (is_func_dedent, is_method) = ctx.function_def_frames.last()
                        .map(|f| (depth == f.initial_depth, f.is_method))
                        .unwrap_or((false, false));
                    
                    if is_func_dedent {
                        if is_method {
                            ctx.emit(Instruction::simple_raw(opcode::RETURN_SELF).0, Some(token));
                        } else {
                            ctx.emit(Instruction::simple_raw(opcode::RETURN).0, Some(token));
                        }
                        
                        let (func_name, function_id) = if let Some(f) = ctx.function_def_frames.last_mut() {
                            let jump_idx = f.start_ip - 1;
                            let target = ctx.instructions.len() as u32;
                            ctx.instructions[jump_idx] = Instruction(ctx.instructions[jump_idx]).with_payload(target).0;

                            let end_ip = ctx.instructions.len() - 1;

                            ctx.value_pool.update_function(
                                f.function_id,
                                f.start_ip,
                                end_ip,
                                f.param_count,
                                f.return_count
                            );

                            (f.name.clone(), f.function_id)
                        } else {
                            (String::new(), 0)
                        };

                        ctx.value_pool.pop_scope();
                        
                        let in_class = ctx.class_frames.last_mut()
                            .filter(|cf| depth == cf.initial_depth + 1)
                            .map(|cf| {
                                cf.method_count += 1;
                                true
                            })
                            .unwrap_or(false);

                        if in_class {
                            if let Some(symbol_ref) = Symbol::from_name(&func_name) {
                                let const_idx = ctx.value_pool.add_to_const_pool(symbol_ref);
                                ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, const_idx).0, Some(token));
                            } else {
                                let const_idx = ctx.value_pool.get_or_create_string(&func_name, &mut ctx.memory_manager);
                                ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, const_idx).0, Some(token));
                            }
                        }

                        let const_idx = ctx.value_pool.get_or_create_function(function_id);
                        ctx.emit(Instruction::new_raw(opcode::LOAD_CONST, const_idx).0, Some(token));
                        
                        if !in_class {
                            ctx.value_pool.get_or_create_var_name(&func_name);
                            if let Some((assign_opcode, idx)) = ctx.value_pool.get_var_assign_opcode(&func_name) {
                                ctx.emit(Instruction::new_raw(assign_opcode, idx).0, Some(token));
                            } else {
                                return Err(pretty_parse_error(ParseErrorType::Custom("Internal error: Failed to resolve function variable scope after creation".to_string()), 1, 1, ctx.source));
                            }
                            ctx.emit(Instruction::simple_raw(opcode::POP).0, Some(token));
                        }

                        ctx.function_def_frames.pop();
                        i += 1;
                        continue;
                    }

                    // Check if this is a class dedent
                    if let Some(cf) = ctx.class_frames.last_mut().filter(|cf| depth == cf.initial_depth) {
                        let (m_count, c_name) = (cf.method_count, cf.name.clone());
                        cf.end_ip = ctx.instructions.len();
                        ctx.emit(Instruction::new_raw(opcode::MAKE_CLASS, m_count).0, Some(token));
                        
                        ctx.value_pool.get_or_create_var_name(&c_name);
                        if let Some((op, idx)) = ctx.value_pool.get_var_assign_opcode(&c_name) {
                            ctx.emit(Instruction::new_raw(op, idx).0, Some(token));
                            ctx.emit(Instruction::simple_raw(opcode::POP).0, Some(token));
                        }
                        
                        ctx.class_frames.pop();
                        i += 1;
                        continue;
                    }

                    let next_is_continuation = tokens.get(i + 1).map_or(false, |t|
                        matches!(t.typ, TT::Elif | TT::Else | TT::Is | TT::In)
                    );

                    let (cond_jump, is_else) = ctx.cond_frames.last()
                        .map(|f| (
                            f.cond_jump,
                            f.current_type == TT::Else
                        ))
                        .unwrap_or((None, false));

                    if !is_else && next_is_continuation {
                        let exit_idx = ctx.instructions.len();
                        ctx.emit(Instruction::new_raw(opcode::JUMP, 0).0, Some(token));
                        if let Some(frame) = ctx.cond_frames.last_mut() {
                            frame.exit_jumps.push(exit_idx);
                        }
                    }

                    if let Some(idx) = cond_jump {
                        let target = ctx.instructions.len() as u32;
                        ctx.instructions[idx] = Instruction(ctx.instructions[idx]).with_payload(target).0;
                    }

                    if !next_is_continuation {
                        let final_pos = ctx.instructions.len() as u32;
                        if let Some(frame) = ctx.cond_frames.last_mut() {
                            for exit_idx in &frame.exit_jumps {
                                ctx.instructions[*exit_idx] = Instruction(ctx.instructions[*exit_idx]).with_payload(final_pos).0;
                            }

                            if frame.count > 0 {
                                /*
                                * CondFrame Lifecycle
                                * ------------------------------
                                * In blocks like If-Is, a unique situation arises where a parent 
                                * conditional block (If) and its child block (Is) share a single CondFrame. 
                                * 
                                * For these cases, the count can be incremented to maintain the 
                                * frame until returning to the parent's Dedent.
                                */
                                frame.count -= 1;
                            } else {
                                if frame.pop_at_end {
                                    // If-Is uses: REF_1 REF_2 DUP(2) EQ JUMP_IF*
                                    // POP is needed to clean up REF_1 at the end of the block
                                    ctx.emit(Instruction::simple_raw(opcode::POP).0, Some(&token));
                                }
                                ctx.cond_frames.pop();
                            }
                            counter.pop();
                        }
                    }

                    i += 1;
                    continue
                }

                TT::End => {
                    main = true;
                    let return_count = 1;
                    Self::pop_to_any_delim(&mut ctx)?;
                    if let Some(top) = ctx.op_stack.pop() {
                        if top.typ == TT::Return {
                            Self::handle_return_statement(&mut ctx, &top, counter.pop())?;
                            i += 1;
                            continue;
                        } else {
                            let error_type = Parser::get_delimiter_error_message(top.typ);
                            return Err(pretty_parse_error(error_type, top.line.unwrap_or(1) as usize, top.column.unwrap_or(1) as usize, ctx.source)); 
                        }
                    } else if let Some(f) = ctx.call_with_depth(depth) {
                        let args = counter.pop();
                        let is_method = f.is_method;
                        Self::create_function_call(&mut ctx, args, is_method)?;
                    }
                    if return_count > 0 {
                        ctx.emit(Instruction::simple_raw(opcode::POP).0, Some(token));
                    }
                    i += 1;
                    continue
                }

                _ => {}
            }
            main = false;
            i += 1;
        }
            
        while let Some(item) = ctx.op_stack.pop() {
            if Parser::STACK_DELIMITERS.contains(&item.typ) {
                let error_type = Parser::get_delimiter_error_message(item.typ);
                return Err(pretty_parse_error(error_type, item.line.unwrap_or(1) as usize, item.column.unwrap_or(1) as usize, ctx.source));
            }
            Self::process_op(item, &mut ctx)?;
        }

        if let Err(e) = Self::register_main_if_allowed(&mut ctx) {
            return Err(e);
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        dinocode_core::native::free_info();

        Ok((
            Bytecode {
                instructions: ctx.instructions,
                memory_manager: ctx.memory_manager,
                const_pool: ctx.value_pool.get_const_pool().to_vec(),
                functions: ctx.value_pool.get_functions().to_vec(),
                global_count: ctx.value_pool.get_global_scope().len() as u32,
                main_function: ctx.main_function,
            },
            ctx.source_map
        ))
    }
}