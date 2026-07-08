// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/bytecode/types.rs
//  Desc:       Bytecode information structures for WASM
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantInfo {
    pub index: u32,
    #[serde(rename = "type")]
    pub const_type: String,
    pub value: String,
    pub raw: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub index: u32,
    pub param_count: u32,
    pub return_count: u32,
    pub start_ip: u32,
    pub end_ip: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionInfo {
    pub ip: u32,
    pub opcode: u8,
    pub opcode_name: String,
    pub operand: u32,
    pub operand_description: String,
    pub source_line: u32,
    pub source_column: u32,
    pub end_ip_for_line: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BytecodeInfo {
    pub constants: Vec<ConstantInfo>,
    pub functions: Vec<FunctionInfo>,
    pub instructions: Vec<InstructionInfo>,
    pub global_count: u32,
    pub instruction_count: u32,
}
