// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/types/context_info.rs
//  Desc:       Additional lexer context for state and positioning
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════

#[derive(Debug)]
pub struct LexerContextInfo {
    // Starting position of the token
    pub from_byte: usize,
    pub from_line: u32,
    pub from_column: u32,
    
    // State flags
    pub had_blank: bool,
    pub blank: bool,
    pub has_dot: bool,
    pub has_underscores: bool,
    pub unary_minus: bool,
    pub unary_plus: bool,
    pub expect_dollar_call: bool,
    pub expect_dot_access: bool,
    pub maybe_dot_access: bool,
    pub is_native_access: bool,
}

impl LexerContextInfo {
    pub fn new() -> Self {
        Self {
            from_byte: 0,
            from_line: 1,
            from_column: 1,
            
            had_blank: false,
            blank: false,
            has_dot: false,
            has_underscores: false,
            unary_minus: false,
            unary_plus: false,
            expect_dollar_call: false,
            expect_dot_access: false,
            maybe_dot_access: false,
            is_native_access: false,
        }
    }

    #[inline(always)]
    pub fn set_may_dot_access(&mut self) {
        self.maybe_dot_access = true;
    }

    #[inline(always)]
    pub fn set_dot_access(&mut self) {
        self.expect_dot_access = true;
    }

    #[inline(always)]
    pub fn reset_dot_access(&mut self) {
        self.expect_dot_access = false;
        self.maybe_dot_access = false;
        self.is_native_access = false;
    }

    #[inline(always)]
    pub fn set_dollar_call(&mut self) {
        self.expect_dollar_call = true;
    }

    #[inline(always)]
    pub fn reset_dollar_call(&mut self) {
        self.expect_dollar_call = false;
    }

    #[inline(always)]
    pub fn reset_number_flags(&mut self) {
        self.has_dot = false;
        self.has_underscores = false;
        self.unary_minus = false;
        self.unary_plus = false;
    } 

    #[inline(always)]
    pub fn has_sign(&self) -> bool {
        self.unary_minus || self.unary_plus
    }

    #[inline(always)]
    pub fn has_dot_access(&self) -> bool {
        self.expect_dot_access || (self.maybe_dot_access && !self.blank)
    }

    #[inline(always)]
    pub fn update_flags(&mut self, i: usize, line: u32, column: u32) {
        self.from_byte = i;
        self.from_line = line;
        self.from_column = column;
        self.had_blank = self.blank;
        self.expect_dollar_call = false;
        self.maybe_dot_access = false;
    }
}
