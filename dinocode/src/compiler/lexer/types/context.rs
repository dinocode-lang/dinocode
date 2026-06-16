// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/types/context.rs
//  Desc:       Lexer context definitions, specially focused on the
//              Golden Rule, parsing modes and indentation handling
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═════════════════════════════════════════════════════════

use crate::shared::utils::Counter;
use super::modes::ParseMode;

#[derive(Debug)]
pub struct LexerContext {
    pub depth: i32,
    pub is_unary: bool,
    pub is_leading: bool,
    pub is_continuous: bool,
    pub is_artificial_break: bool,
    pub allow_indent: bool,
    pub allow_redirect: bool,
    pub indent_counter: Counter,
    pub current_indent: usize,
    pub current_mode: ParseMode,
    pub parse_stack: Vec<ParseMode>,
}

impl LexerContext {
    pub fn new() -> Self {
        Self {
            depth: 0,
            is_unary: false,
            is_leading: true,
            is_continuous: false,
            is_artificial_break: false,
            allow_indent: false,
            allow_redirect: false,
            indent_counter: Counter::new(),
            current_indent: 0,
            current_mode: ParseMode::Normal,
            parse_stack: vec![ParseMode::Normal],
        }
    }

    pub fn current_mode(&self) -> ParseMode {
        *self.parse_stack.last().unwrap_or(&ParseMode::Normal)
    }

    pub fn push_mode(&mut self, mode: ParseMode) {
        self.parse_stack.push(mode);
        self.current_mode = mode;
    }

    pub fn pop_mode(&mut self) {
        self.parse_stack.pop();
        self.current_mode = self.current_mode();
    }

    #[inline(always)]
    pub fn join_next(&mut self) {
        self.is_continuous = true;
    }

    #[inline(always)]
    pub fn break_next(&mut self) {
        self.is_artificial_break = true;
    }

    #[inline(always)]
    pub fn shift_break(&mut self) {
        if self.is_artificial_break {
            self.join_next();
            self.is_unary = false;
        }
    }

    #[inline(always)]
    pub fn discard_break(&mut self) {
        if self.is_artificial_break {
            self.is_artificial_break = false;
            self.is_unary = self.is_continuous;
        }
    }

    #[inline(always)]
    pub fn is_artificial_unary(&self) -> bool {
        self.is_unary && self.is_artificial_break
    }

    #[inline(always)]
    pub fn update_unary(&mut self, tokens_len: usize) {
        self.is_unary = self.is_continuous || self.is_artificial_break || tokens_len == 0;
    }
    
    #[inline(always)]
    pub fn start_indent(&mut self) {
        self.allow_indent = true;
    }
    
    #[inline(always)]
    pub fn stop_indent(&mut self) {
        self.allow_indent = false;
    }

     #[inline(always)]
    pub fn start_redirect(&mut self) {
        self.allow_redirect = true;
    }

     #[inline(always)]
    pub fn stop_redirect(&mut self) {
        self.allow_redirect = false;
    }
}
