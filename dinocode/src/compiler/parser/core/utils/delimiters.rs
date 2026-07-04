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

use crate::{
    shared::types::{
        TokenType,
        Token,
    },
    compiler::parser::{
        errors::{
            ParseError,
            ParseErrorType,
        },
        types::ParserContext,
        core::Parser,
    }
};

use TokenType as TT;

impl Parser {
    pub fn get_delimiter_error_message(delim_type: TokenType) -> ParseErrorType {
        match delim_type {
            TT::LParen => ParseErrorType::ExpectedToken(")"),
            TT::LBrace => ParseErrorType::ExpectedToken("}"),
            TT::LBracket => ParseErrorType::ExpectedToken("]"),
            TT::LString => ParseErrorType::ExpectedToken("string terminator"),
            TT::LBraceExpr => ParseErrorType::ExpectedToken("}" ),
            TT::RParen => ParseErrorType::ExpectedToken("("),
            TT::RBrace => ParseErrorType::ExpectedToken("{"),
            TT::RBracket => ParseErrorType::ExpectedToken("["),
            TT::RString => ParseErrorType::ExpectedToken("string initializer"),
            TT::RBraceExpr => ParseErrorType::ExpectedToken("{"),

            TT::IsMatch => ParseErrorType::ExpectedIndentedBlock("is"),
            TT::InMatch => ParseErrorType::ExpectedIndentedBlock("in"),
            TT::If => ParseErrorType::ExpectedIndentedBlock("if"),
            TT::Else => ParseErrorType::ExpectedIndentedBlock("else"),
            TT::Elif => ParseErrorType::ExpectedIndentedBlock("elif"),
            TT::While => ParseErrorType::ExpectedIndentedBlock("while"),
            TT::For => ParseErrorType::ExpectedIndentedBlock("for"),
            TT::Is => ParseErrorType::ExpectedIndentedBlock("is"),
            TT::In => ParseErrorType::ExpectedIndentedBlock("in"),
            TT::Function => ParseErrorType::ExpectedIndentedBlock("function"),
            TT::Class => ParseErrorType::ExpectedIndentedBlock("class"),

            _ => ParseErrorType::MismatchedDelimiter("unrecognized delimiter"),
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
         Err(ParseError::from_token(
             error_type,
             token
         ))
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
                    return Err(ParseError::new(
                        ParseErrorType::MismatchedDelimiter("mismatched delimiter"),
                        top.line.unwrap_or(1),
                        top.column.unwrap_or(1)
                    ));
            }
            if Self::process_op(ctx.op_stack.pop().unwrap(), ctx)? {
                break;
            }
        }
        Ok(())
    }
}
