// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/memory/types/pool_types.rs
//  Desc:       Native types managed by object pool
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::types::DinoRef;
pub use super::property_flags::{
    PropertyFlags,
    PROP_NORMAL,
    PROP_GETTER,
    PROP_SETTER,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeArray {
    pub capacity: u32,
    pub count: u32,
    pub elements: *mut DinoRef,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeObject {
    pub capacity: u32,
    pub count: u32,
    pub entries: *mut ObjectEntry,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ObjectEntry {
    pub hash: u64,
    pub key: u64,
    pub value: DinoRef,
    pub flags: u8,
}

#[repr(C)]
pub union PoolSlotData {
    pub array:     NativeArray,
    pub object:    NativeObject,
    pub free_next: u32,
}

#[repr(C)]
pub struct PoolSlot {
    pub kind: u16,
    pub subkind: u16,
    pub proto: DinoRef,
    pub data: PoolSlotData,
}
