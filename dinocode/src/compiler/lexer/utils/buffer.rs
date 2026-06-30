// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/utils/buffer.rs
//  Desc:       Buffer processing utilities for the lexer
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    shared::{
        types::{
            Token,
            TokenType,
            Operator,
            STATEMENT_KEYWORDS,
        },
        errors::{
            pretty_lex_error,
            LexErrorType,
            LexError,
        },
        utils::Operators,
    },
    compiler::lexer::{
        types::{
            LexerContextInfo,
            TokenList,
            LexerContext,
            BufType,
            ParseMode,
        },
        utils::numeric::{
            parse_i64_lex,
            parse_f64_lex,
        }
    },
};

#[inline(always)]
pub fn push_number(
    source: &str,
    end_pos: usize,
    info: &mut LexerContextInfo,
    tokens: &mut TokenList,
    ctx: &mut LexerContext,
) -> Result<(), LexError> {
    let raw = if info.has_underscores {
        &source[info.from_byte..end_pos].replace("_", "")
    } else {
        &source[info.from_byte..end_pos]
    };
    
    let token = if info.has_dot {
        match parse_f64_lex(raw, info.unary_minus) {
            Ok(parsed_value) => {
                Token::float(parsed_value, Some((info.from_line, info.from_column, end_pos - info.from_byte)))
            }
            Err(err) => {
                return Err(pretty_lex_error(
                    err, 
                    info.from_line as usize, 
                    info.from_column as usize, 
                    source
                ));
            }
        }
    } else {
        match parse_i64_lex(raw, info.unary_minus) {
            Ok(parsed_value) => {
                Token::integer(parsed_value, Some((info.from_line, info.from_column, end_pos - info.from_byte)))
            }
            Err(err) => {
                return Err(pretty_lex_error(
                    err, 
                    info.from_line as usize, 
                    info.from_column as usize, 
                    source
                ));
            }
        }
    };

    tokens.push(token, ctx);
    Ok(())
}

#[inline(always)]
pub fn push_hex(
    source: &str,
    end_pos: usize,
    info: &mut LexerContextInfo,
    tokens: &mut TokenList,
    ctx: &mut LexerContext,
) -> Result<(), LexError> {
    let raw = if info.has_underscores {
        &source[info.from_byte..end_pos].replace("_", "")
    } else {
        &source[info.from_byte..end_pos]
    };
    
    match parse_i64_lex(raw, info.unary_minus) {
        Ok(parsed_value) => {
            tokens.push(Token::integer(parsed_value, Some((info.from_line, info.from_column, end_pos - info.from_byte))), ctx);
            Ok(())
        }
        Err(err) => {
            return Err(pretty_lex_error(
                err, 
                info.from_line as usize, 
                info.from_column as usize, 
                source
            ));
        }
    }
}

#[inline(always)]
pub fn push_bit(
    source: &str,
    end_pos: usize,
    info: &mut LexerContextInfo,
    tokens: &mut TokenList,
    ctx: &mut LexerContext,
) -> Result<(), LexError> {
    let raw = if info.has_underscores {
        &source[info.from_byte..end_pos].replace("_", "")
    } else {
        &source[info.from_byte..end_pos]
    };
    
    match parse_i64_lex(raw, info.unary_minus) {
        Ok(parsed_value) => {
            tokens.push(Token::integer(parsed_value, Some((info.from_line, info.from_column, end_pos - info.from_byte))), ctx);
            Ok(())
        }
        Err(err) => {
            return Err(pretty_lex_error(
                err, 
                info.from_line as usize, 
                info.from_column as usize, 
                source
            ));
        }
    }
}

#[inline(always)]
pub fn push_octal(
    source: &str,
    end_pos: usize,
    info: &mut LexerContextInfo,
    tokens: &mut TokenList,
    ctx: &mut LexerContext,
) -> Result<(), LexError> {
    let raw = if info.has_underscores {
        &source[info.from_byte..end_pos].replace("_", "")
    } else {
        &source[info.from_byte..end_pos]
    };
    
    match parse_i64_lex(raw, info.unary_minus) {
        Ok(parsed_value) => {
            tokens.push(Token::integer(parsed_value, Some((info.from_line, info.from_column, end_pos - info.from_byte))), ctx);
            Ok(())
        }
        Err(err) => {
            return Err(pretty_lex_error(
                err, 
                info.from_line as usize, 
                info.from_column as usize, 
                source
            ));
        }
    }
}

#[inline(always)]
pub fn push_bigint(
    source: &str,
    end_pos: usize,
    info: &mut LexerContextInfo,
    tokens: &mut TokenList,
    ctx: &mut LexerContext,
) -> Result<(), LexError> {
    let raw = if info.has_underscores {
        &source[info.from_byte..end_pos].replace("_", "")
    } else {
        &source[info.from_byte..end_pos]
    };
    tokens.push(Token::bigint(raw.to_string(), Some((info.from_line, info.from_column, end_pos - info.from_byte))), ctx);
    Ok(())
}

#[inline(always)]
pub fn push_scientific(
    source: &str,
    end_pos: usize,
    info: &mut LexerContextInfo,
    tokens: &mut TokenList,
    ctx: &mut LexerContext,
) -> Result<(), LexError> {
    let raw = if info.has_underscores {
        &source[info.from_byte..end_pos].replace("_", "")
    } else {
        &source[info.from_byte..end_pos]
    };
    
    match parse_f64_lex(raw, info.unary_minus) {
        Ok(parsed_value) => {
            tokens.push(Token::float(parsed_value, Some((info.from_line, info.from_column, end_pos - info.from_byte))), ctx);
            Ok(())
        }
        Err(err) => {
            Err(pretty_lex_error(
                err, 
                info.from_line as usize, 
                info.from_column as usize, 
                source
            ))
        }
    }
}

#[inline(always)]
pub fn push_ident(
    source: &str,
    end_pos: usize,
    info: &mut LexerContextInfo,
    ctx: &mut LexerContext,
    tokens: &mut TokenList,
) -> Result<(), LexError> {
    let ident = &source[info.from_byte..end_pos];
    let ident_lower = ident.to_lowercase();

    if !ctx.is_leading && STATEMENT_KEYWORDS.contains(&ident_lower.as_str()) {
        return Err(pretty_lex_error(
            LexErrorType::ReservedKeywordAsIdentifier, 
            info.from_line as usize, 
            info.from_column as usize, 
            source
        ));
    }

    match ident_lower.as_str() {
        "true" => tokens.push(Token::bool(true, Some((info.from_line, info.from_column, 4))), ctx),
        "false" => tokens.push(Token::bool(false, Some((info.from_line, info.from_column, 5))), ctx),
        "none" => tokens.push(Token::none(Some((info.from_line, info.from_column, 4))), ctx),
        "nan" => tokens.push(Token::nan(Some((info.from_line, info.from_column, 3))), ctx),
        "infi" => tokens.push(Token::infi(Some((info.from_line, info.from_column, 4))), ctx),
        "and" | "or" | "not" | "as" => process_op(&ident_lower, source, end_pos, ctx, info, tokens)?,
        "if" => {
            tokens.push(Token::delim(TokenType::If, Some((info.from_line, info.from_column, 2))), ctx);
            ctx.start_indent();
            ctx.break_next();
        },
        "elif" => {
            tokens.push(Token::delim(TokenType::Elif, Some((info.from_line, info.from_column, 4))), ctx);
            ctx.start_indent();
            ctx.break_next();
        },
        "else" => {
            tokens.push(Token::delim(TokenType::Else, Some((info.from_line, info.from_column, 4))), ctx);
            ctx.start_indent();
        },
        "is" => {
            tokens.push(Token::delim(TokenType::Is, Some((info.from_line, info.from_column, 2))), ctx);
            ctx.start_indent();
            ctx.break_next();
        },
        "in" => {
            tokens.push(Token::delim(TokenType::In, Some((info.from_line, info.from_column, 2))), ctx);
            ctx.start_indent();
            ctx.break_next();
        },
        "while" => {
            tokens.push(Token::delim(TokenType::While, Some((info.from_line, info.from_column, 5))), ctx);
            ctx.start_indent();
            ctx.break_next();
        },
        "for" => {
            tokens.push(Token::delim(TokenType::For, Some((info.from_line, info.from_column, 3))), ctx);
            ctx.start_indent();
        },
        "$" => {
            info.set_dollar_call();
        },
        "return" => {
            tokens.push(Token::delim(TokenType::Return, Some((info.from_line, info.from_column, 6))), ctx);
            ctx.break_next();
        },
        "break" => {
            tokens.push(Token::delim(TokenType::Break, Some((info.from_line, info.from_column, 5))), ctx);
            ctx.break_next();
        },
        "continue" => {
            tokens.push(Token::delim(TokenType::Continue, Some((info.from_line, info.from_column, 8))), ctx);
            ctx.break_next();
        },
        _ => {
            let was_leading = ctx.is_leading;
            let token = if info.has_dot_access() {
                let is_native_access = info.is_native_access;
                info.reset_dot_access();
                if is_native_access {
                    Token::native_access(ident, Some((info.from_line, info.from_column, ident.len())))
                } else {
                    Token::dot_access(ident, Some((info.from_line, info.from_column, ident.len())))
                }
            } else {
                Token::identifier(ident, Some((info.from_line, info.from_column, ident.len())))
            };

            tokens.push(token, ctx);
            
            if was_leading { ctx.break_next(); }
        }
    }
    
    Ok(())
}

#[inline(always)]
pub fn push_dollar(
    source: &str,
    end_pos: usize,
    info: &mut LexerContextInfo,
    tokens: &mut TokenList,
    ctx: &mut LexerContext,
) {
    let ident = &source[info.from_byte..end_pos];
    tokens.push(
        Token::dollar_call(ident, Some((info.from_line, info.from_column, ident.len()))),
        ctx
    );
    ctx.break_next();
}

#[inline(always)]
pub fn process_op(
    op_str: &str,
    source: &str,
    end_pos: usize,
    ctx: &mut LexerContext,
    info: &mut LexerContextInfo,
    tokens: &mut TokenList,
) -> Result<(), LexError> {
    match Operators::from_str(op_str) {
        Ok(op) => {
            let is_unary = ctx.is_unary;
            let is_leading = ctx.is_leading;
            let mut is_breaker = false;
            let mut is_unary_allowed = false;
            
            match op {
                Operator::Gt if is_leading => {
                    /*
                    * Literal block redirection
                    * ------------------------------------------------
                    * Finalizes any previous statement and marks the start of a 
                    * redirection header.
                    * 
                    * Example:
                    * > VAR_NAME
                    *   ----------
                    *   Hello world
                    *   ----------
                    */
                    tokens.push(Token::redirect(Some((info.from_line, info.from_column, end_pos - info.from_byte))), ctx);
                    ctx.start_redirect();
                    return Ok(());
                }    
                Operator::BiColon if is_leading => {
                    tokens.push(Token::class(Some((info.from_line, info.from_column, end_pos - info.from_byte))), ctx);
                    ctx.start_indent();
                    return Ok(());
                }
                Operator::Colon if is_leading => {
                    tokens.push(Token::function(Some((info.from_line, info.from_column, end_pos - info.from_byte))), ctx);
                    ctx.start_indent();
                    return Ok(());
                }

                Operator::Add if is_unary => {
                    info.unary_plus = true; 
                    is_unary_allowed = true;
                }

                Operator::Sub if is_unary => {
                    info.unary_minus = true; 
                    is_unary_allowed = true;
                }

                Operator::Not => {
                    is_unary_allowed = true;
                    ctx.is_unary = true;    // It can only be a unary operator
                }

                Operator::Inc | Operator::Dec => {
                    /*
                    * Operator adjacency
                    * --------------------------------------
                    * Discards artificial breaks when an operator is physically adjacent to the 
                    * preceding token. This prevents the parser from misinterpreting 
                    * the operator as a prefix unary for an implicit call's argument.
                    *
                    * Examples:
                    * print ++x    # '++' is prefix unary for 'x'
                    * print++      # Discard break, '++' stays adjacent to 'print'
                    *
                    * Note: discard_break() already handles the restoration of the unary state
                    *       so update_unary() is not necessary.
                    */
                    if ctx.is_artificial_unary() && !info.had_blank {
                        ctx.discard_break();
                    }
                    
                    is_unary_allowed = true;
                    is_breaker = !ctx.is_unary;
                }

                Operator::Dot | Operator::Backdot => {
                    info.is_native_access = op == Operator::Backdot;

                    if ctx.is_artificial_unary() {
                        /*
                        * Dot operator in unary context
                        * ------------------------------------------------
                        * The dot operator as unary can only be valid for a property or method access.
                        *
                        * Example:
                        * .method()     # '.' is part of method call, not unary
                        * .property     # '.' is part of property access, not unary
                        *
                        * Special example:
                        * object        # The statement is pending termination.
                        *    .property  # This would normally terminate the previous statement, but now it chains to it
                        */
                        ctx.shift_break();  // It only postpones the artificial break to a next potential implicit first argument
                        info.set_dot_access();
                        return Ok(());
                    }

                    info.set_may_dot_access();
                }

                _ if ctx.is_artificial_unary() && Operators::is_assign(op) => {
                    /*
                    * Assignment operators in unary context
                    * ------------------------------------------------
                    * This override prevents assignment operators from being
                    * misclassified as unary in an implicit function call.
                    *
                    * Example:
                    * print -5      # '-' acts as unary
                    * print = 5     # '=' acts as binary
                    *
                    * Note: discard_break() already handles the restoration of the unary state
                    *       so update_unary() is not necessary.
                    */
                    ctx.discard_break();
                }

                _ => {}
            }

            if ctx.is_unary && !is_unary_allowed {
                return Err(pretty_lex_error(LexErrorType::OperatorNotAllowed, info.from_line as usize, info.from_column as usize, source));
            }

            tokens.push(
                Token::op(op, Some((info.from_line, info.from_column, end_pos - info.from_byte)))
                    .with_unary(ctx.is_unary)
                    .with_breaker(is_breaker),
                ctx
            );

            Ok(())
        }
        Err(_e) => Err(pretty_lex_error(LexErrorType::InvalidOperator, info.from_line as usize, info.from_column as usize, source))
    }
}

#[inline(always)]
pub fn push_operator(
    source: &str,
    end_pos: usize,
    info: &mut LexerContextInfo,
    tokens: &mut TokenList,
    ctx: &mut LexerContext,
) -> Result<(), LexError> {
    let op_str = &source[info.from_byte..end_pos];
    process_op(op_str, source, end_pos, ctx, info, tokens)
}

#[inline(always)]
pub fn push_buffer(
    source: &str,
    end_pos: usize,
    buf_type: BufType,
    line: u32,
    info: &mut LexerContextInfo,
    tokens: &mut TokenList,
    ctx: &mut LexerContext,
    string_buf: &mut String,
) -> Result<(), LexError> {
    if let ParseMode::InBlockString(_) = ctx.current_mode {
        if !string_buf.is_empty() {
            if string_buf.ends_with('\n') {
                string_buf.pop();
            }
            if string_buf.ends_with('\r') {
                string_buf.pop();
            }
            tokens.push(Token::string(string_buf.clone(), Some((line, info.from_column, string_buf.len()))), ctx);
            string_buf.clear();
        }
        tokens.push(Token::delim(TokenType::RString, Some((line, info.from_column, 0))), ctx);
        ctx.pop_mode();
        ctx.is_leading = true;
        return Ok(());
    }

    match buf_type {
        BufType::Number => push_number(source, end_pos, info, tokens, ctx),
        BufType::HexNumber => push_hex(source, end_pos, info, tokens, ctx),
        BufType::BitNumber => push_bit(source, end_pos, info, tokens, ctx),
        BufType::OctalNumber => push_octal(source, end_pos, info, tokens, ctx),
        BufType::ScientificNumber => push_scientific(source, end_pos, info, tokens, ctx),
        BufType::Identifier => push_ident(source, end_pos, info, ctx, tokens),
        BufType::DollarCall => {
            Err(pretty_lex_error(
                LexErrorType::UnexpectedDollarCall, 
                info.from_line as usize, 
                info.from_column as usize, 
                source
            ))
        }
        BufType::Operator => {
            let op_str = &source[info.from_byte..end_pos];
            match Operators::from_str(op_str) {
                Ok(op) => {
                    if Operators::can_breaker(op) {
                        if ctx.is_artificial_unary() && !info.had_blank {
                            ctx.discard_break();
                        }
                        
                        tokens.push(
                            Token::op(op, Some((info.from_line, info.from_column, end_pos - info.from_byte)))
                                .with_unary(ctx.is_unary)
                                .with_breaker(true),
                            ctx
                        );
                        Ok(())
                    } else {
                        Err(pretty_lex_error(
                            LexErrorType::InvalidOperator, 
                            info.from_line as usize, 
                            info.from_column as usize, 
                            source
                        ))
                    }
                }
                Err(_) => Err(pretty_lex_error(LexErrorType::InvalidOperator, info.from_line as usize, info.from_column as usize, source))
            }
        }
        BufType::None 
        | BufType::String 
        | BufType::FString
        | BufType::Comment => Ok(()),
        BufType::MultiLineComment => {
            Err(pretty_lex_error(
                LexErrorType::Custom("Unterminated multiline comment".to_string()), 
                info.from_line as usize, 
                info.from_column as usize, 
                source
            ))
        }
    }
}

