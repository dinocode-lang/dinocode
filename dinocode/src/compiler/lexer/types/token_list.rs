// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/types/token_list.rs
//  Desc:       Token collection and management utilities.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::shared::types::{
    Token,
    TokenType,
};
use crate::compiler::lexer::types::LexerContext;

#[derive(Debug, Clone, Default)]
pub struct TokenList {
    tokens: Vec<Token>,
}

impl TokenList {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn push(&mut self, token: Token, ctx: &mut LexerContext) {
        if !(self.tokens.is_empty() || ctx.is_continuous || token.is_delimiter()) {
            if ctx.is_leading {
                if ctx.allow_indent && ctx.current_indent > ctx.indent_counter.get() {
                    ctx.indent_counter.push(ctx.current_indent);
                    self.tokens.push(Token::delim(TokenType::Indent, None));
                    ctx.stop_indent();
                } else {
                    self.tokens.push(Token::end(None));
                    while ctx.current_indent < ctx.indent_counter.get() {
                        ctx.indent_counter.leave();
                        self.tokens.push(Token::delim(TokenType::Dedent, None));
                    }
                }
            } else {
                self.tokens.push(Token::comma());
            }
            ctx.discard_break();
        }
        ctx.is_continuous = token.is_continuous();
        ctx.is_leading = false;
        self.tokens.push(token);
    }

    pub fn push_raw(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn last(&self) -> Option<&Token> {
        self.tokens.last()
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }
    
    pub fn iter(&self) -> std::slice::Iter<'_, Token> {
        self.tokens.iter()
    }

    pub fn pop(&mut self) -> Option<Token> {
        self.tokens.pop()
    }
}
