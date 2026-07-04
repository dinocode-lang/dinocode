// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/core/lexer.rs
//  Desc:       Handles tokenization of source code into tokens.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    shared::types::{
        Token,
        TokenType,
        Operator,
    },
    compiler::lexer::{
        errors::{
            LexError,
            LexErrorType,
        },
        types::{
            BufType,
            ParseMode,
            LexerContext,
            LexerContextInfo,
            TokenList,
        },
        utils::{
            string::{
                handle_escape,
                handle_interpolation,
                push_utf8_char,
            },
            headers::get_header_len,
            buffer::{
                push_number, 
                push_hex, 
                push_bit, 
                push_octal,
                push_bigint,
                push_scientific, 
                push_ident, 
                push_dollar,
                push_operator,
                push_buffer,
            },
            char_utils::{
                is_ident_start,
                is_op_start,
                is_ident,
                is_digit,
                is_hex_digit,
                is_binary_digit,
                is_octal_digit,
                is_sci_exp,
                is_sci_digit,
                is_bigint_posfix,
            },
        },
    }
};

pub struct Lexer;

impl Lexer {

    pub fn tokenize(
        source: &str
    ) -> Result<TokenList, LexError> {
        let bytes = source.as_bytes();
        let bytes_len = bytes.len();
        let mut tokens = TokenList::new();
        let mut string_buf = String::new();

        let mut i = get_header_len(bytes);
        let mut column: u32 = 1;
        let mut line: u32 = 1;

        let mut ctx = LexerContext::new();
        let mut info = LexerContextInfo::new();
        let mut buf_type = BufType::None;
        let mut op_info: (&[u8], usize) = (b"", 0);

        macro_rules! pos {
            () => { Some((info.from_line, info.from_column, i - info.from_byte)) };
        }

        macro_rules! pos_now {
            () => { Some((line, column, 1)) };
        }

        macro_rules! has_next {
            () => {{
                i + 1 < bytes_len
            }};
        }

        macro_rules! next_is {
            ($expected:expr) => {{
                i + 1 < bytes_len && bytes[i + 1] == $expected
            }};
        }

        while i < bytes_len {
            let b = bytes[i];

            match ctx.current_mode {
                ParseMode::Normal | ParseMode::InFStringExpr(_) => {
                    match buf_type {
                        BufType::Number => {
                            if is_digit(b) {}
                            else if b == b'_' {
                                info.has_underscores = true;
                            } else if b == b'.' && !info.has_dot && !next_is!(b'.') {
                                info.has_dot = true;
                            } else if b == b'x' {
                                buf_type = BufType::HexNumber;
                            } else if b == b'b' {
                                buf_type = BufType::BitNumber;
                            } else if b == b'o' {
                                buf_type = BufType::OctalNumber;
                            } else if is_sci_exp(b) {
                                buf_type = BufType::ScientificNumber;
                            } else {
                                let is_bigint = is_bigint_posfix(b);
                                if is_bigint {
                                    push_bigint(source, i, &mut info, &mut tokens, &mut ctx)?;
                                } else {
                                    push_number(source, i, &mut info, &mut tokens, &mut ctx)?;
                                }
                                
                                buf_type = BufType::None;
                                info.reset_number_flags();

                                if is_bigint {
                                    i += 1;
                                    column += 1;
                                    continue;
                                }
                            }
                        }
                        BufType::HexNumber => {
                            if is_hex_digit(b) {}
                            else if b == b'_' {
                                info.has_underscores = true;
                            } else {
                                let is_bigint = is_bigint_posfix(b);
                                if is_bigint {
                                    push_bigint(source, i, &mut info, &mut tokens, &mut ctx)?;
                                } else {
                                    push_hex(source, i, &mut info, &mut tokens, &mut ctx)?;
                                };
                                
                                buf_type = BufType::None;
                                info.reset_number_flags();

                                if is_bigint {
                                    i += 1;
                                    column += 1;
                                    continue;
                                }
                            }
                        }
                        BufType::BitNumber => {
                            if is_binary_digit(b) {}
                            else if b == b'_' {
                                info.has_underscores = true;
                            } else {
                                let is_bigint = is_bigint_posfix(b);
                                if is_bigint {
                                    push_bigint(source, i, &mut info, &mut tokens, &mut ctx)?;
                                } else {
                                    push_bit(source, i, &mut info, &mut tokens, &mut ctx)?;
                                };
                                
                                buf_type = BufType::None;
                                info.reset_number_flags();

                                if is_bigint {
                                    i += 1;
                                    column += 1;
                                    continue;
                                }
                            }
                        }
                        BufType::OctalNumber => {
                            if is_octal_digit(b) {}
                            else if b == b'_' {
                                info.has_underscores = true;
                            } else {
                                let is_bigint = is_bigint_posfix(b);
                                if is_bigint {
                                    push_bigint(source, i, &mut info, &mut tokens, &mut ctx)?;
                                } else {
                                    push_octal(source, i, &mut info, &mut tokens, &mut ctx)?;
                                };
                                
                                buf_type = BufType::None;
                                info.reset_number_flags();

                                if is_bigint {
                                    i += 1;
                                    column += 1;
                                    continue;
                                }
                            }
                        }
                        BufType::ScientificNumber => {
                            if is_sci_digit(b) {}
                            else if b == b'_' {
                                info.has_underscores = true;
                            } else {
                                push_scientific(source, i, &mut info, &mut tokens, &mut ctx)?;
                                
                                buf_type = BufType::None;
                                info.reset_number_flags();
                            }
                        }
                        BufType::Identifier => {
                            if !is_ident(b) {
                                push_ident(source, i, &mut info, &mut ctx, &mut tokens)?;
                                buf_type = BufType::None;
                            }
                        }

                        BufType::DollarCall => {
                            if !is_ident(b) {
                                push_dollar(source, i, &mut info, &mut tokens, &mut ctx);
                                buf_type = BufType::None;
                            }
                        }

                        BufType::Operator => {
                            if !(i - info.from_byte < op_info.1 && op_info.0.contains(&b)) {
                                push_operator(source, i, &mut info, &mut tokens, &mut ctx)?;
                                buf_type = BufType::None;
                            }
                        }
                        BufType::Comment => {
                            if b == b'\n' {
                                buf_type = BufType::None;
                            }
                        }
                        BufType::MultiLineComment => {
                            if b == b'*' && next_is!(b'#') {
                                i += 1;
                                column += 1;
                                buf_type = BufType::None;
                                continue
                            } else if b == b'\n' {
                                line += 1;
                                column = 1;
                            }
                        }
                        _ => {}
                    }

                    if buf_type == BufType::None {
                        let mut is_relevant = true;
                        match b {
                            b' ' | b'\t' => {
                                if ctx.is_leading { 
                                    ctx.current_indent += if b == b' ' { 1 } else { 4 };
                                }
                                info.blank = true;
                                i += 1;
                                column += 1;
                                continue;
                            }
                            b'\r' => { i += 1; continue; }
                            b'\n' => {
                                if ctx.depth == 0 && !ctx.is_continuous {
                                    if ctx.allow_redirect {
                                        /*
                                        * Literal Block Redirection ('>' Operator)
                                        * ------------------------------------------------
                                        * Handles the implicit assignment of a multi-line block
                                        *
                                        * Example:
                                        * > VAR_NAME
                                        *   ----------
                                        *   Hello world
                                        *   ----------
                                        * 
                                        * Becomes:
                                        * VAR_NAME = "---\nHello world\n---"
                                        */
                                        tokens.push(
                                            Token::op(Operator::Assign, pos_now!())
                                                .with_unary(false),
                                            &mut ctx
                                        );

                                        ctx.push_mode(ParseMode::InBlockString(ctx.current_indent));
                                        tokens.push(Token::delim(TokenType::LString, pos!()), &mut ctx);
                                        string_buf.clear();

                                        ctx.stop_redirect();
                                    } else {
                                        ctx.is_leading = true;
                                    }
                                    ctx.current_indent = 0;
                                } else if ctx.allow_redirect {
                                    /*
                                     * Line integrity
                                     * ------------------------------
                                     * The redirection header (the expression and the '>') must be 
                                     * contiguous. This error triggers if a line break occurs before 
                                     * the header expression is fully resolved, preventing detached 
                                     * or ambiguous redirections.
                                     */
                                    return Err(LexError::new(LexErrorType::UnexpectedLogicalContinuation, info.from_line as u32, info.from_column as u32));
                                }
                                line += 1;
                                info.blank = true;
                                i += 1;
                                column = 1;
                                continue;
                            }
                            b';' => {
                                if ctx.is_continuous {
                                    return Err(LexError::new(LexErrorType::UnexpectedToken("unexpected semicolon"), info.from_line as u32, column as u32));
                                }
                                ctx.current_indent = 0;
                                ctx.is_leading = true;
                                tokens.handle_virtual_delims(false, pos_now!(), &mut ctx);
                                ctx.join_next(); // Prevents the inclusion of another End
                                i += 1;
                                column += 1;
                                continue;
                            }
                            b'(' => {
                                if !info.blank {
                                    if info.expect_dollar_call  {
                                        is_relevant = false; // Should be postponed until the next token (Identifier) 
                                    } else {
                                        ctx.join_next(); 
                                        ctx.discard_break(); // Prevent the propagation of the unnecessary artificial break in this context
                                    }
                                }
                                tokens.push(Token::delim(TokenType::LParen, pos_now!()), &mut ctx);
                                ctx.depth += 1;
                            }
                            b')' => { 
                                tokens.push(Token::delim(TokenType::RParen, pos_now!()), &mut ctx); 
                                ctx.depth -= 1;
                            }
                            b'[' => { 
                                if !info.blank {
                                    ctx.join_next(); 
                                    ctx.discard_break(); // Prevent the propagation of the unnecessary artificial break in this context
                                }
                                tokens.push(Token::delim(TokenType::LBracket, pos_now!()), &mut ctx); 
                                ctx.depth += 1; 
                            }
                            b']' => { 
                                tokens.push(Token::delim(TokenType::RBracket, pos_now!()), &mut ctx); 
                                ctx.depth -= 1; 
                            }
                            b'{' => { 
                                if let Some(ParseMode::InFStringExpr(depth)) = ctx.parse_stack.last_mut() {
                                    *depth += 1;
                                }
                                tokens.push(Token::delim(TokenType::LBrace, pos_now!()), &mut ctx); 
                                ctx.depth += 1; 
                            }
                            b'}' => {
                                if let Some(ParseMode::InFStringExpr(depth)) = ctx.parse_stack.last_mut() {
                                    if *depth > 0 {
                                        *depth -= 1;
                                        tokens.push(Token::delim(TokenType::RBrace, pos_now!()), &mut ctx);
                                        i += 1;
                                        column += 1;
                                        continue;
                                    } else if ctx.parse_stack.len() > 1 {
                                        tokens.push(Token::delim(TokenType::RBraceExpr, pos_now!()), &mut ctx);
                                        ctx.pop_mode();
                                        ctx.depth -= 1;
                                        i += 1;
                                        column += 1;
                                        continue;
                                    }
                                }
                                tokens.push(Token::delim(TokenType::RBrace, pos_now!()), &mut ctx);
                                ctx.depth -= 1;
                            }
                            b',' => { 
                                tokens.push(Token::delim(TokenType::Comma, pos_now!()), &mut ctx);
                                ctx.discard_break();  // Prevent the propagation of the unnecessary artificial break in this context
                            }
                            b'"' => {
                                ctx.push_mode(ParseMode::InFString);
                                tokens.push(Token::delim(TokenType::LString, pos_now!()), &mut ctx);
                                string_buf.clear();
                            }
                            b'\'' => {
                                ctx.push_mode(ParseMode::InString);
                                tokens.push(Token::delim(TokenType::LString, pos_now!()), &mut ctx);
                                string_buf.clear();
                            }
                            b'0'..=b'9' => { 
                                buf_type = BufType::Number; 
                            }
                            b'#' => { 
                                buf_type = if next_is!(b'*') { BufType::MultiLineComment } else { BufType::Comment };
                                is_relevant = false;
                            }
                            _ => {
                                ctx.update_unary(tokens.len());
                                if is_ident_start(b) {
                                    buf_type = if info.expect_dollar_call { BufType::DollarCall } else { BufType::Identifier };
                                } else if let Some(info) = is_op_start(b) {
                                    buf_type = BufType::Operator;
                                    op_info = info;
                                } else {
                                    return Err(LexError::new(LexErrorType::UnexpectedToken("unexpected character"), info.from_line as u32, column as u32))
                                }
                            }
                        }

                        if is_relevant {
                            if info.expect_dollar_call && buf_type != BufType::DollarCall {
                                if info.blank {
                                    return Err(LexError::new(LexErrorType::DollarCallWithSpace, info.from_line as u32, info.from_column as u32));
                                }
                                return Err(LexError::new(LexErrorType::UnexpectedToken("unexpected dollar call"), info.from_line as u32, info.from_column as u32));
                            }
                            if info.has_dot_access() {
                                if buf_type != BufType::Identifier {
                                    return Err(LexError::new(LexErrorType::UnexpectedTokenAfterDot, info.from_line as u32, info.from_column as u32));
                                } else if info.blank {
                                    return Err(LexError::new(LexErrorType::UnexpectedBlankAfterDot, info.from_line as u32, info.from_column as u32));
                                }
                                if info.maybe_dot_access {
                                    tokens.pop();   // Discard the dot operator
                                }
                                info.expect_dot_access = true;  // Will be resolved in push_ident
                            }
                            
                            if info.has_sign() {
                                if buf_type == BufType::Number {
                                    tokens.pop();   // Discard the unary (-) operator, as it would be injected directly into the number
                                } else {
                                    info.reset_number_flags();
                                }
                            }

                            info.update_flags(i, line, column);
                        }
                        info.blank = false;
                    }
                }

                ParseMode::InFString => {
                    match b {
                        b'\\' if has_next!() => {
                            i += 1;
                            column += 1;
                            handle_escape(bytes[i], &mut string_buf);
                        }
                        b'\n' => {
                            column = 1;
                            line += 1;
                            string_buf.push('\n');
                        }
                        b'$' => {    
                            handle_interpolation(
                                bytes,
                                source,
                                &mut i, 
                                line,
                                &mut column, 
                                &info, 
                                &mut string_buf, 
                                &mut tokens, 
                                &mut ctx
                            );
                        }
                        b'"' => {
                            if next_is!(b'"') {
                                string_buf.push('"');
                                i += 2;
                                column += 2;
                                continue;
                            }
                            if !string_buf.is_empty() {
                                tokens.push(Token::string(string_buf.clone(), Some((line, info.from_column, string_buf.len()))), &mut ctx);
                                string_buf.clear();
                            }
                            ctx.pop_mode();
                            tokens.push(Token::delim(TokenType::RString, pos!()), &mut ctx);
                        }
                        _ => {
                            let bytes_consumed = push_utf8_char(&mut string_buf, bytes, i);
                            i += bytes_consumed;
                            column += bytes_consumed as u32;
                            continue;
                        }
                    }
                } 
                
                ParseMode::InBlockString(base_indent) => {
                    if ctx.current_indent == 0 {
                        let mut local_indent = 0;
                        while i < bytes_len
                            && local_indent <= base_indent
                            && (bytes[i] == b' ' || bytes[i] == b'\t')
                        {
                            local_indent += if bytes[i] == b' ' { 1 } else { 4 };
                            i += 1;
                            column += 1;
                        }

                        ctx.current_indent = local_indent;

                        if local_indent <= base_indent {
                            if !string_buf.is_empty() {
                                if string_buf.ends_with('\n') {
                                    string_buf.pop();
                                }
                                if string_buf.ends_with('\r') {
                                    string_buf.pop();
                                }
                                tokens.push(Token::string(string_buf.clone(), Some((line, info.from_column, string_buf.len()))), &mut ctx);
                                string_buf.clear();
                            } else {
                                return Err(LexError::new(LexErrorType::EmptyLiteralBlockString, info.from_line as u32, info.from_column as u32));
                            }
                            tokens.push(Token::delim(TokenType::RString, pos!()), &mut ctx);
                            ctx.pop_mode();
                            ctx.is_leading = true; // Next token should be a statement start
                        }
                        continue
                    }

                    match b {
                        b'\n' => {
                            line += 1;
                            column = 1;
                            ctx.current_indent = 0;
                            string_buf.push('\n');

                        }
                        b'$' => {    
                            handle_interpolation(
                                bytes,
                                source,
                                &mut i, 
                                line,
                                &mut column, 
                                &info, 
                                &mut string_buf, 
                                &mut tokens, 
                                &mut ctx
                            );
                        }
                        _ => {
                            let bytes_consumed = push_utf8_char(&mut string_buf, bytes, i);
                            i += bytes_consumed;
                            column += bytes_consumed as u32;
                            continue;
                        }
                    }
                } 

                ParseMode::InString => {
                    match b {
                        b'\n' => {
                            column = 1;
                            line += 1;
                            string_buf.push('\n');
                        }
                        b'\'' => {
                            if next_is!(b'\'') {
                                string_buf.push('\'');
                                i += 2;
                                column += 2;
                                continue;
                            }
                            tokens.push(Token::string(string_buf.clone(), Some((line, info.from_column, string_buf.len()))), &mut ctx);
                            tokens.push(Token::delim(TokenType::RString, pos!()), &mut ctx);
                            string_buf.clear();
                            ctx.pop_mode();
                        }
                        _ => {
                            let bytes_consumed = push_utf8_char(&mut string_buf, bytes, i);
                            i += bytes_consumed;
                            column += bytes_consumed as u32;
                            continue;
                        }
                    }
                }
            }
            i += 1;
            column += 1; 
        }

        push_buffer(source, bytes_len, buf_type, line, &mut info, &mut tokens, &mut ctx, &mut string_buf)?;

        if !tokens.is_empty() && tokens.last().map_or(false, |t| t.typ != TokenType::End) {
            tokens.push(Token::end(pos_now!()), &mut ctx);
        }
        
        while ctx.indent_counter.get() > 0 {
            ctx.indent_counter.leave();
            tokens.push_raw(Token::delim(TokenType::Dedent, pos_now!()));
        }

        Ok(tokens)
    }
}
