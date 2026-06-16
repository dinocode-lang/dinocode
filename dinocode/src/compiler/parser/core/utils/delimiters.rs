// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/core/utils/delimiters.rs
//  Desc:       Delimiter matching and stack management utilities.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::shared::{
    types::{
        TokenType,
        Token,
    },
    errors::{
        ParseError,
        ParseErrorType,
        pretty_parse_error,
    },
};
use crate::compiler::parser::{
    types::ParserContext,
    core::Parser,
};

use TokenType as TT;

impl Parser {
    pub fn get_delimiter_error_message(delim_type: TokenType) -> ParseErrorType {
        match delim_type {
            TT::LParen => ParseErrorType::ExpectedRightParen,
            TT::LBrace => ParseErrorType::ExpectedRightBrace,
            TT::LBracket => ParseErrorType::ExpectedRightBracket,
            TT::LString => ParseErrorType::ExpectedStringTerminator,
            TT::LBraceExpr => ParseErrorType::ExpectedRightBraceExpr,
            TT::RParen => ParseErrorType::ExpectedLeftParen,
            TT::RBrace => ParseErrorType::ExpectedLeftBrace,
            TT::RBracket => ParseErrorType::ExpectedLeftBracket,
            TT::RString => ParseErrorType::ExpectedStringInitializer,
            TT::RBraceExpr => ParseErrorType::ExpectedLeftBraceExpr,

            TT::IsMatch => ParseErrorType::ExpectedIndentedBlock(TT::Is),
            TT::InMatch => ParseErrorType::ExpectedIndentedBlock(TT::In),
            TT::If | TT::Else | TT::Elif |
            TT::While | TT::For | TT::Is | TT::In |
            TT::Function | TT::Class => ParseErrorType::ExpectedIndentedBlock(delim_type),

            _ => ParseErrorType::MismatchedDelimiter(format!("Mismatched delimiter: {:?}", delim_type)),
        }
    }

    pub fn get_counteract_delim(delim_type: TokenType) -> TokenType {
        match delim_type {
            TT::LParen => TT::RParen,
            TT::LBrace => TT::RBrace,
            TT::LBracket => TT::RBracket,
            TT::LString => TT::RString,
            TT::LBraceExpr => TT::RBraceExpr,
            TT::RParen => TT::LParen,
            TT::RBrace => TT::LBrace,
            TT::RBracket => TT::LBracket,
            TT::RString => TT::LString,
            TT::RBraceExpr => TT::LBraceExpr,
            _ => delim_type,
        }
    }

    pub fn is_continuous(i: usize, tokens: &[Token]) -> bool {
        if i == 0 || i > tokens.len() {
            return false;
        }

        let t = &tokens[i - 1];

        !(Parser::STACK_DELIMITERS.contains(&t.typ) 
            || t.typ == TT::Comma 
            || matches!(t.typ, TT::Op(_)))
    }

    pub fn pop_until_delim(
        ctx: &mut ParserContext,
        target: TokenType,
        token: &Token,
    ) -> Result<(), ParseError> {
        while let Some(item) = ctx.op_stack.pop() {
            if item.typ == target {
                return Ok(());
            }
            if Self::process_op(item, ctx)? {
                return Ok(());
            }
        }
         let error_type = Self::get_delimiter_error_message(Self::get_counteract_delim(target));
         Err(pretty_parse_error(error_type, token.line.unwrap_or(1) as usize, token.column.unwrap_or(1) as usize, ctx.source))
    }

    pub fn pop_to_any_delim(
        ctx: &mut ParserContext,
    ) -> Result<(), ParseError> {
        while let Some(top) = ctx.op_stack.last() {
            if Parser::STACK_DELIMITERS.contains(&top.typ) { break; }
            if Self::process_op(ctx.op_stack.pop().unwrap(), ctx)? {
                break;
            }
        }
        Ok(())
    }

    pub fn pop_to_delim<F>(
        ctx: &mut ParserContext,
        is_target: F,
    ) -> Result<(), ParseError> 
    where 
        F: Fn(TokenType) -> bool 
    {
        while let Some(top) = ctx.op_stack.last() {
            if is_target(top.typ) { break; }
            if Parser::STACK_DELIMITERS.contains(&top.typ) {
                    let error_type = Parser::get_delimiter_error_message(top.typ.clone());
                    return Err(pretty_parse_error(error_type, top.line.unwrap_or(1) as usize, top.column.unwrap_or(1) as usize, ctx.source));
            }
            if Self::process_op(ctx.op_stack.pop().unwrap(), ctx)? {
                break;
            }
        }
        Ok(())
    }
}
