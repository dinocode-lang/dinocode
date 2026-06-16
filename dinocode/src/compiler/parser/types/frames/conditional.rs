// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/frames/conditional.rs
//  Desc:       Frame for parsing if/elif/else and loops.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::shared::types::TokenType;

#[derive(Debug)]
pub struct CondFrame {
    pub cond_jump: Option<usize>,
    pub exit_jumps: Vec<usize>,
    pub if_jumps: Vec<usize>,
    pub current_type: TokenType,
    pub loop_start: Option<usize>,
    pub temp_vars_to_cleanup: Vec<(u8, u32)>,
    pub expr_count: usize,
    pub count: usize,
    pub pop_at_end: bool,
}
