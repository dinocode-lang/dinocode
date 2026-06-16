// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/opcode.rs
//  Desc:       Utility functions for opcodes.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::types::opcode_defs::opcode::*;

pub fn opcode_name(op: u8) -> &'static str {
    match op {
        NOP => "NOP",
        HALT => "HALT",
        
        LOAD_CONST => "LOAD_CONST",
        GET_LOCAL => "GET_LOCAL",
        GET_GLOBAL => "GET_GLOBAL",
        SET_LOCAL => "SET_LOCAL",
        SET_GLOBAL => "SET_GLOBAL",
        DROP_LOCAL => "DROP_LOCAL",
        DROP_GLOBAL => "DROP_GLOBAL",
        TRUE => "TRUE",
        FALSE => "FALSE",
        NONE => "NONE",
        
        JUMP => "JUMP",
        JUMP_IF => "JUMP_IF",
        JUMP_IF_NOT => "JUMP_IF_NOT",
        RETURN => "RETURN",
        RETURN_REF => "RETURN_REF",
        RETURN_SELF => "RETURN_SELF",
        CALL => "CALL",
        FOR_INIT => "FOR_INIT",
        FOR_ITER => "FOR_ITER",
        FOR_ITER_ARRAY => "FOR_ITER_ARRAY",
        FOR_ITER_RANGE => "FOR_ITER_RANGE",
        FOR_ITER_STRING => "FOR_ITER_STRING",
        FOR_DROP => "FOR_DROP",
        
        POP => "POP",
        POP_N => "POP_N",
        DUP => "DUP",
        
        ADD => "ADD",
        SUB => "SUB",
        MUL => "MUL",
        DIV => "DIV",
        FLOOR_DIV => "FLOOR_DIV",
        MOD => "MOD",
        POW => "POW",
        
        EQ => "EQ",
        NE => "NE",
        GT => "GT",
        LT => "LT",
        GE => "GE",
        LE => "LE",
        
        BIT_AND => "BIT_AND",
        BIT_OR => "BIT_OR",
        BIT_XOR => "BIT_XOR",
        DOT => "DOT",
        
        NOT => "NOT",
        NEG => "NEG",
        BIT_NOT => "BIT_NOT",
        
        TO => "TO",
        
        MAKE_ARRAY => "MAKE_ARRAY",
        MAKE_OBJECT => "MAKE_OBJECT",
        GET_MEMBER => "GET_MEMBER",
        GET_MEMBER_PREP => "GET_MEMBER_PREP",
        SET_MEMBER => "SET_MEMBER",
        SET_MEMBER_PREP => "SET_MEMBER_PREP",
        GET_METHOD => "GET_METHOD",
        STR_BUILD => "STR_BUILD",
        MAKE_CLASS => "MAKE_CLASS",
        MAKE_RANGE => "MAKE_RANGE",
        GET_NATIVE_MEMBER => "GET_NATIVE_MEMBER",
        GET_NATIVE_METHOD => "GET_NATIVE_METHOD",
        
        GET_INDEX => "GET_INDEX",
        GET_INDEX_PREP => "GET_INDEX_PREP",
        SET_INDEX => "SET_INDEX",
        SET_INDEX_PREP => "SET_INDEX_PREP",
        INPUT => "INPUT",
        IN => "IN",
        
        _ => "UNKNOWN",
    }
}

pub fn opcode_symbol(op: u8) -> &'static str {
    match op {        
        ADD => "+",
        SUB | NEG => "-",
        MUL => "*",
        DIV => "/",
        FLOOR_DIV => "//",
        MOD => "%",
        POW => "**",
        
        EQ => "==",
        NE => "!=",
        GT => ">",
        LT => "<",
        GE => ">=",
        LE => "<=",
        
        BIT_AND => "&",
        BIT_OR => "|",
        BIT_XOR => "^",
        DOT => ".",
        
        IN => "in",
        
        NOT => "!",
        BIT_NOT => "~",
        
        _ => "UNKNOWN",
    }
}
