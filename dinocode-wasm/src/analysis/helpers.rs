// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/analysis/helpers.rs
//  Desc:       Helper analysis functions for flowchart generation
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::bytecode::types::BytecodeInfo;
use dinocode_core::{
    types::{dinoref::DinoRef},
    native::get_native_registry,
};
use dinocode_core::types::opcode_defs::opcode::{
    CALL,
    LOAD_CONST,
    GET_LOCAL,
    GET_GLOBAL,
    TRUE,
    FALSE,
    NONE,
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    FLOOR_DIV,
    MOD,
    POW,
    EQ,
    NE,
    GT,
    LT,
    GE,
    LE,
    BIT_AND,
    BIT_OR,
    BIT_XOR,
    GET_INDEX,
    GET_MEMBER,
    GET_METHOD,
    MAKE_RANGE,
    NOT,
    NEG,
    BIT_NOT,
    STR_BUILD,
    MAKE_ARRAY,
    MAKE_OBJECT,
    SET_LOCAL,
    SET_GLOBAL,
    INPUT,
};

pub fn get_print_ref() -> Option<DinoRef> {
    get_native_registry()
        .get_id_by_name_unchecked("print")
        .map(DinoRef::native_fn)
}

fn get_stack_effect(opcode: u8, operand: u32) -> (i32, i32) {
    match opcode {
        LOAD_CONST | GET_LOCAL | GET_GLOBAL | TRUE | FALSE | NONE => (0, 1),
        POP => (1, 0),
        ADD | SUB | MUL | DIV | FLOOR_DIV | MOD | POW | EQ | NE | GT | LT | GE | LE | BIT_AND | BIT_OR | BIT_XOR | GET_INDEX | GET_MEMBER | GET_METHOD | MAKE_RANGE => (2, 1),
        NOT | NEG | BIT_NOT => (1, 1),
        STR_BUILD | MAKE_ARRAY => (operand as i32, 1),
        MAKE_OBJECT => (2 * (operand as i32), 1),
        CALL => ((operand as i32) + 1, 1),
        SET_LOCAL | SET_GLOBAL => (1, 0),
        INPUT => (2, 1),
        _ => (0, 0),
    }
}

pub fn is_print_call(info: &BytecodeInfo, ip: u32) -> Option<u32> {
    let instr = info.instructions.get(ip as usize)?;
    if instr.opcode != CALL {
        return None;
    }
    let argc = instr.operand;
    
    let mut needed = (argc as i32) + 1;
    let mut func_ref_ip = None;
    
    for prev_ip in (0..ip).rev() {
        if let Some(prev_instr) = info.instructions.get(prev_ip as usize) {
            let (pop, push) = get_stack_effect(prev_instr.opcode, prev_instr.operand);
            needed = needed - push + pop;
            if needed <= 0 {
                func_ref_ip = Some(prev_ip);
                break;
            }
        }
    }
    
    if let Some(ref_ip) = func_ref_ip {
        if let Some(func_ref_instr) = info.instructions.get(ref_ip as usize) {
            if func_ref_instr.opcode == LOAD_CONST {
                let const_idx = func_ref_instr.operand as usize;
                if let Some(constant) = info.constants.get(const_idx) {
                    if let Some(raw) = constant.raw {
                        if let Some(print_ref) = get_print_ref() {
                            if raw == print_ref.raw() {
                                return Some(argc);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn get_source_range(
    source: &str,
    start_line: u32,
    start_col: u32,
    end_line: u32,
    end_col: u32,
) -> String {
    if start_line == 0 || end_line == 0 || start_line > end_line {
        return String::new();
    }
    let lines: Vec<&str> = source.lines().collect();
    let mut result = Vec::new();
    for line_num in start_line..=end_line {
        if let Some(line) = lines.get((line_num - 1) as usize) {
            let mut line_str = *line;
            if line_num == end_line {
                let col_limit = (end_col as usize).min(line_str.len());
                line_str = &line_str[..col_limit];
            }
            if line_num == start_line {
                let col_start = (start_col.saturating_sub(1) as usize).min(line_str.len());
                line_str = &line_str[col_start..];
            }
            result.push(line_str);
        }
    }
    result.join("\n").trim().to_string()
}
