// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/types/instruction.rs
//  Desc:       Instruction wrapper and basic operations
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instruction(pub u32);

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instr({:?}, idx={})", self.opcode_byte(), self.payload())
    }
}

impl Instruction {
    const MASK_OP: u32 = 0xFF00_0000;
    const SHIFT_OP: u32 = 24;
    const MASK_PAYLOAD: u32 = !Self::MASK_OP;

    #[inline(always)]
    pub fn new_raw(opcode: u8, payload: u32) -> Self {
        debug_assert!(payload <= Self::MASK_PAYLOAD, "Payload too large for instruction");
        let raw = ((opcode as u32) << Self::SHIFT_OP) | (payload & Self::MASK_PAYLOAD);
        Instruction(raw)
    }

    #[inline(always)]
    pub fn simple_raw(opcode: u8) -> Self {
        Self::new_raw(opcode, 0)
    }

    #[inline(always)]
    pub fn opcode_byte(&self) -> u8 {
        ((self.0 >> Self::SHIFT_OP) & 0xFF) as u8
    }

    #[inline(always)]
    pub fn payload(&self) -> u32 {
        self.0 & Self::MASK_PAYLOAD
    }

    #[inline(always)]
    pub fn with_payload(&self, new_payload: u32) -> Self {
        Instruction((self.0 & Self::MASK_OP) | (new_payload & Self::MASK_PAYLOAD))
    }
}
