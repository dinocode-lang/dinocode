// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/conversions/to_bool.rs
//  Desc:       Boolean type conversions for TypeConverter
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    errors::{RuntimeError, RuntimeErrorType},
    types::{DinoRef, value_type},
};
use super::TypeConverter;

impl TypeConverter {
    pub fn to_primitive_bool(value: DinoRef, vtype: u16, memory: &mut MemoryManager) -> Result<bool, RuntimeError> {
        match vtype {
            value_type::INT => Ok(value.as_int() != 0),
            value_type::FLOAT => Ok(value.as_non_nan_float()? != 0.0),
            value_type::BIGINT => {
                let bigint = memory.get_bigint(value.as_bigint());
                Ok(!bigint.is_zero())
            }
            value_type::STRING => {
                let offset = value.decode_index();
                Ok(memory.get_const_len(offset) > 0)
            }
            value_type::BOOL => Ok(value.as_bool()),
            value_type::NONE => Ok(false),
            value_type::SYMBOL => Ok(true),
            value_type::ARRAY => {
                let id = value.decode_index();
                Ok(memory.get_array_len(id) > 0)
            }
            value_type::OBJECT => {
                let id = value.get_object_id();
                Ok(memory.get_object_len(id) > 0)
            }
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvert {
                from: value.type_name().to_string(),
                to: "bool".to_string(),
                info: None,
                help: None,
            })),
        }
    }

    pub fn to_bool(value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        let vtype = value.decode_type();
        if vtype == value_type::BOOL { return Ok(value); }
        let b = Self::to_primitive_bool(value, vtype, memory)?;
        Ok(if b { DinoRef::TRUE } else { DinoRef::FALSE })
    }
}
