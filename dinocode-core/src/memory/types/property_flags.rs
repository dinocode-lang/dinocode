// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/types/property_flags.rs
//  Desc:       Property flags management
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

pub const PROP_NORMAL: u8 = 0x00;
pub const PROP_GETTER: u8 = 0x01;
pub const PROP_SETTER: u8 = 0x02;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyFlags(u8);

impl PropertyFlags {
    #[inline(always)]
    pub const fn new(flags: u8) -> Self {
        Self(flags)
    }
    
    #[inline(always)]
    pub const fn normal() -> Self {
        Self(PROP_NORMAL)
    }
    
    #[inline(always)]
    pub const fn getter() -> Self {
        Self(PROP_GETTER)
    }
    
    #[inline(always)]
    pub const fn setter() -> Self {
        Self(PROP_SETTER)
    }
    
    #[inline(always)]
    pub const fn getter_setter() -> Self {
        Self(PROP_GETTER | PROP_SETTER)
    }
    
    #[inline(always)]
    pub fn is_getter(&self) -> bool {
        self.0 & PROP_GETTER != 0
    }
    
    #[inline(always)]
    pub fn is_setter(&self) -> bool {
        self.0 & PROP_SETTER != 0
    }
    
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        self.0 == PROP_NORMAL
    }
    
    #[inline(always)]
    pub fn has_any(&self, flag: u8) -> bool {
        self.0 & flag != 0
    }
    
    #[inline(always)]
    pub fn raw(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PropertyFlags {
    #[inline(always)]
    fn from(flags: u8) -> Self {
        Self(flags)
    }
}

impl From<PropertyFlags> for u8 {
    #[inline(always)]
    fn from(flags: PropertyFlags) -> Self {
        flags.0
    }
}
