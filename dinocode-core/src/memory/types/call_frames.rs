// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/types/call_frames.rs
//  Desc:       Call stack frame structure
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub return_address: u32,
    pub old_bp: usize,
    pub function_id: u32,
    pub args_start: usize,
    pub args_count: usize,
    pub return_count: u32,
}
