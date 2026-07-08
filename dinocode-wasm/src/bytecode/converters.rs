// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/bytecode/converters.rs
//  Desc:       Conversion utilities for bytecode information
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::bytecode::types::{
    ConstantInfo,
    FunctionInfo,
    InstructionInfo,
    BytecodeInfo,
};
use dinocode_core::{
    utils::{
        opcode::opcode_name,
        SourceMap,
    },
    types::{
        opcode_defs::opcode::{
            LOAD_CONST,
            GET_LOCAL,
            SET_LOCAL,
            GET_GLOBAL,
            SET_GLOBAL,
            JUMP,
            JUMP_IF,
            JUMP_IF_NOT,
            CALL,
            MAKE_ARRAY,
            MAKE_OBJECT,
            MAKE_CLASS,
        },
        dinoref::DinoRef,
        UserFunction,
    },
    memory::MemoryManager,
};
use dinocode::compiler::parser::types::Bytecode;

impl ConstantInfo {
    pub fn from_dinoref(index: u32, dino_ref: &DinoRef, memory: &mut MemoryManager) -> Self {
        let const_type = dino_ref.type_name();
        let value = dino_ref
            .try_as_string(memory)
            .map(|s| {
                if s.chars().count() > 50 {
                    let mut r = String::with_capacity(53);
                    r.extend(s.chars().take(50));
                    r.push_str("...");
                    r
                } else {
                    s
                }
            })
            .unwrap_or("<unknown>".to_string());

        Self {
            index,
            const_type: const_type.to_string(),
            value,
            raw: Some(dino_ref.raw()),
        }
    }
}

impl FunctionInfo {
    pub fn from_user_function(index: u32, func: &UserFunction) -> Self {
        Self {
            index,
            param_count: func.param_count,
            return_count: func.return_count,
            start_ip: func.start_ip as u32,
            end_ip: func.end_ip as u32,
        }
    }
}

impl InstructionInfo {
    pub fn from_bytecode(
        ip: u32,
        instruction: u32,
        bytecode: &Bytecode,
        source_map: &SourceMap,
    ) -> Self {
        let opcode_byte = ((instruction >> 24) & 0xFF) as u8;
        let payload = instruction & 0x00FFFFFF;
        
        let op_name = opcode_name(opcode_byte).to_string();
        let operand_desc = Self::decode_operand(opcode_byte, payload, bytecode);
        
        let (source_line, source_column, _, end_ip_for_line) = source_map
            .get_location_and_range(ip as usize)
            .unwrap_or((0, 0, 0, 0));

        Self {
            ip,
            opcode: opcode_byte,
            opcode_name: op_name,
            operand: payload,
            operand_description: operand_desc,
            source_line: source_line as u32,
            source_column: source_column as u32,
            end_ip_for_line: end_ip_for_line as u32,
        }
    }

    fn decode_operand(opcode: u8, operand: u32, bytecode: &Bytecode) -> String {
        let mut s = String::with_capacity(32);
        match opcode {
            LOAD_CONST => {
                if (operand as usize) < bytecode.const_pool.len() {
                    format!("#{} {:?}", operand, bytecode.const_pool[operand as usize])
                } else {
                    s.push("#".chars().next().unwrap());
                    s.push_str(&operand.to_string());
                    s.push_str(" (out of range)");
                    s
                }
            }
            GET_LOCAL | SET_LOCAL => {
                s.push_str("local[");
                s.push_str(&operand.to_string());
                s.push("]".chars().next().unwrap());
                s
            }
            GET_GLOBAL | SET_GLOBAL => {
                s.push_str("global[");
                s.push_str(&operand.to_string());
                s.push("]".chars().next().unwrap());
                s
            }
            JUMP | JUMP_IF | JUMP_IF_NOT => {
                s.push_str("-> ");
                s.push_str(&operand.to_string());
                s
            }
            CALL => {
                s.push_str("argc: ");
                s.push_str(&operand.to_string());
                s
            }
            MAKE_ARRAY | MAKE_OBJECT | MAKE_CLASS => {
                s.push_str("size: ");
                s.push_str(&operand.to_string());
                s
            }
            _ => {
                if operand > 0 {
                    operand.to_string()
                } else {
                    String::new()
                }
            }
        }
    }
}

impl BytecodeInfo {
    pub fn from_bytecode_and_source_map(
        bytecode: &mut Bytecode,
        source_map: &SourceMap,
    ) -> Self {
        let constants: Vec<ConstantInfo> = bytecode
            .const_pool
            .iter()
            .enumerate()
            .map(|(idx, dino_ref)| {
                ConstantInfo::from_dinoref(idx as u32, dino_ref, &mut bytecode.memory_manager)
            })
            .collect();

        let functions: Vec<FunctionInfo> = bytecode
            .functions
            .iter()
            .enumerate()
            .map(|(idx, func)| FunctionInfo::from_user_function(idx as u32, func))
            .collect();

        let instructions: Vec<InstructionInfo> = bytecode
            .instructions
            .iter()
            .enumerate()
            .map(|(ip, &instruction)| {
                InstructionInfo::from_bytecode(ip as u32, instruction, bytecode, source_map)
            })
            .collect();

        Self {
            constants,
            functions,
            instructions,
            global_count: bytecode.global_count,
            instruction_count: bytecode.instructions.len() as u32,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn get_instructions_at_line(&self, line: u32) -> Vec<&InstructionInfo> {
        self.instructions
            .iter()
            .filter(|instr| instr.source_line == line)
            .collect()
    }

    pub fn get_function_range(&self, function_index: u32) -> Option<(u32, u32)> {
        self.functions.get(function_index as usize).map(|func| {
            (func.start_ip, func.end_ip)
        })
    }

    pub fn get_instruction_at(&self, ip: u32) -> Option<&InstructionInfo> {
        self.instructions.get(ip as usize)
    }
}
