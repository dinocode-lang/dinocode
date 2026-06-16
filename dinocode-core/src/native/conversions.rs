// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/native/conversions.rs
//  Desc:       Type conversion traits for #dinof macro.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════
use crate::{
    memory::MemoryManager,
    types::{DinoRef, dinoref::value_type},
    errors::{Result, RuntimeError, RuntimeErrorType},
};

pub trait FromDinoRef {
    fn from_dinoref(arg: DinoRef, memory: &MemoryManager) -> Result<Self>
    where
        Self: Sized;
}

pub trait ToDinoRef {
    fn to_dinoref(self, memory: &mut MemoryManager) -> Result<DinoRef>;
}

impl FromDinoRef for i64 {
    fn from_dinoref(arg: DinoRef, _memory: &MemoryManager) -> Result<Self> {
        match arg.decode_type() {
            value_type::INT => Ok(arg.as_int()),
            value_type::FLOAT => Ok(arg.as_float() as i64),
            value_type::BOOL => Ok(if arg.as_bool() { 1 } else { 0 }),
            _ => Err(RuntimeError::Typed(RuntimeErrorType::ExpectedInteger(arg.type_name().to_string()))),
        }
    }
}

impl FromDinoRef for i32 {
    fn from_dinoref(arg: DinoRef, _memory: &MemoryManager) -> Result<Self> {
        match arg.decode_type() {
            value_type::INT => Ok(arg.as_int() as i32),
            value_type::FLOAT => Ok(arg.as_float() as i32),
            value_type::BOOL => Ok(if arg.as_bool() { 1 } else { 0 }),
            _ => Err(RuntimeError::Typed(RuntimeErrorType::ExpectedInteger(arg.type_name().to_string()))),
        }
    }
}

impl FromDinoRef for f64 {
    fn from_dinoref(arg: DinoRef, _memory: &MemoryManager) -> Result<Self> {
        match arg.decode_type() {
            value_type::INT => Ok(arg.as_int() as f64),
            value_type::FLOAT => Ok(arg.as_float()),
            value_type::BOOL => Ok(if arg.as_bool() { 1.0 } else { 0.0 }),
            _ => Err(RuntimeError::Typed(RuntimeErrorType::ExpectedNumber(arg.type_name().to_string()))),
        }
    }
}

impl FromDinoRef for bool {
    fn from_dinoref(arg: DinoRef, _memory: &MemoryManager) -> Result<Self> {
        match arg.decode_type() {
            value_type::BOOL => Ok(arg.as_bool()),
            value_type::INT => Ok(arg.as_int() != 0),
            value_type::FLOAT => Ok(arg.as_float() != 0.0),
            value_type::NONE => Ok(false),
            _ => Ok(true),
        }
    }
}

impl FromDinoRef for String {
    fn from_dinoref(arg: DinoRef, memory: &MemoryManager) -> Result<Self> {
        match arg.decode_type() {
            value_type::STRING => Ok(memory.get_string(arg.decode_index()).to_string()),
            value_type::INT => Ok(arg.as_int().to_string()),
            value_type::FLOAT => Ok(arg.as_float().to_string()),
            value_type::BOOL => Ok(if arg.as_bool() { "true" } else { "false" }.to_string()),
            value_type::NONE => Ok("none".to_string()),
            _ => Ok(format!("{:?}", arg)),
        }
    }
}

impl ToDinoRef for i64 {
    fn to_dinoref(self, _memory: &mut MemoryManager) -> Result<DinoRef> {
        Ok(DinoRef::int(self))
    }
}

impl ToDinoRef for i32 {
    fn to_dinoref(self, _memory: &mut MemoryManager) -> Result<DinoRef> {
        Ok(DinoRef::int(self as i64))
    }
}

impl ToDinoRef for f64 {
    fn to_dinoref(self, _memory: &mut MemoryManager) -> Result<DinoRef> {
        Ok(DinoRef::float(self))
    }
}

impl ToDinoRef for bool {
    fn to_dinoref(self, _memory: &mut MemoryManager) -> Result<DinoRef> {
        Ok(DinoRef::bool(self))
    }
}

impl ToDinoRef for &str {
    fn to_dinoref(self, memory: &mut MemoryManager) -> Result<DinoRef> {
        Ok(memory.alloc_string(self))
    }
}

impl ToDinoRef for String {
    fn to_dinoref(self, memory: &mut MemoryManager) -> Result<DinoRef> {
        Ok(memory.alloc_string(&self))
    }
}

impl ToDinoRef for () {
    fn to_dinoref(self, _memory: &mut MemoryManager) -> Result<DinoRef> {
        Ok(DinoRef::NONE)
    }
}
