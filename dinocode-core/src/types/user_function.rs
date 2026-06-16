// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/types/user_function.rs
//  Desc:       User function type definitions.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct UserFunction {
    pub is_main: bool,
    pub start_ip: usize,
    pub end_ip: usize,
    pub param_count: u32,
    pub return_count: u32,
    pub local_count: u32,
}
