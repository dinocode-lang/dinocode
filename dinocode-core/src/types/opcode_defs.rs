// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/types/opcode_defs.rs
//  Desc:       Opcode constants for the VM.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

pub mod opcode {
    pub const NOP: u8 = 0x00;
    pub const HALT: u8 = 0x01;

    pub const LOAD_CONST: u8 = 0x10;
    pub const GET_LOCAL: u8 = 0x16;
    pub const GET_GLOBAL: u8 = 0x17;
    pub const SET_LOCAL: u8 = 0x18;
    pub const SET_GLOBAL: u8 = 0x19;
    pub const DROP_LOCAL: u8 = 0x1A;
    pub const DROP_GLOBAL: u8 = 0x1B;
    pub const TRUE: u8 = 0x13;
    pub const FALSE: u8 = 0x14;
    pub const NONE: u8 = 0x15;
    
    pub const JUMP: u8 = 0x20;
    pub const JUMP_IF: u8 = 0x21;
    pub const JUMP_IF_NOT: u8 = 0x22;
    pub const RETURN: u8 = 0x23;
    pub const RETURN_REF: u8 = 0x24;
    pub const RETURN_SELF: u8 = 0x25;
    pub const CALL: u8 = 0x26;
    pub const FOR_INIT: u8 = 0x27;
    pub const FOR_ITER: u8 = 0x28;
    pub const FOR_ITER_ARRAY: u8 = 0x29;
    pub const FOR_ITER_RANGE: u8 = 0x2A;
    pub const FOR_ITER_STRING: u8 = 0x2B;
    pub const FOR_DROP: u8 = 0x2C;

    pub const POP: u8 = 0x30;
    pub const POP_N: u8 = 0x31;
    pub const DUP: u8 = 0x32;

    pub const ADD: u8 = 0x40;
    pub const SUB: u8 = 0x41;
    pub const MUL: u8 = 0x42;
    pub const DIV: u8 = 0x43;
    pub const FLOOR_DIV: u8 = 0x44;
    pub const MOD: u8 = 0x45;
    pub const POW: u8 = 0x46;
    
    pub const EQ: u8 = 0x47;
    pub const NE: u8 = 0x48;
    pub const GT: u8 = 0x49;
    pub const LT: u8 = 0x4A;
    pub const GE: u8 = 0x4B;
    pub const LE: u8 = 0x4C;
    
    pub const BIT_AND: u8 = 0x4D;
    pub const BIT_OR: u8 = 0x4E;
    pub const BIT_XOR: u8 = 0x4F;
    pub const DOT: u8 = 0x50;
    
    pub const NOT: u8 = 0x60;
    pub const NEG: u8 = 0x61;
    pub const BIT_NOT: u8 = 0x62;
    
    pub const TO: u8 = 0x63;

    pub const MAKE_ARRAY: u8 = 0x70;
    pub const MAKE_OBJECT: u8 = 0x71;
    pub const GET_MEMBER: u8 = 0x72;
    pub const GET_MEMBER_PREP: u8 = 0x73;
    pub const SET_MEMBER: u8 = 0x74;
    pub const SET_MEMBER_PREP: u8 = 0x75;
    pub const GET_METHOD: u8 = 0x76;
    pub const STR_BUILD: u8 = 0x77;
    pub const MAKE_CLASS: u8 = 0x78;
    pub const MAKE_RANGE: u8 = 0x79;
    pub const GET_NATIVE_MEMBER: u8 = 0x7A;
    pub const GET_NATIVE_METHOD: u8 = 0x7B;
    
    pub const GET_INDEX: u8 = 0x7C;
    pub const GET_INDEX_PREP: u8 = 0x7D;
    pub const SET_INDEX: u8 = 0x7E;
    pub const SET_INDEX_PREP: u8 = 0x7F;
    pub const INPUT: u8 = 0x80;
    pub const IN: u8 = 0x81;

    pub const UNKNOWN: u8 = 0xFF;
}
