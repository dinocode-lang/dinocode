// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/conversions/to_string.rs
//  Desc:       String type conversions for TypeConverter.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    errors::{RuntimeError, RuntimeErrorType},
    utils::dinojson,
    types::{DinoRef, value_type, Symbol},
};
use super::TypeConverter;

impl TypeConverter {
    pub fn to_primitive_string(value: DinoRef, vtype: u16, memory: &mut MemoryManager) -> Result<String, RuntimeError> {
        match vtype {
            value_type::INT => Ok(value.as_int().to_string()),
            value_type::FLOAT => Ok(value.as_float().to_string()),
            value_type::BIGINT => Ok(memory.get_bigint(value.decode_index()).to_string()),
            value_type::STRING => Ok(memory.get_string(value.decode_index()).to_string()),
            value_type::BOOL => Ok(value.as_bool().to_string()),
            value_type::NONE => Ok("none".to_string()),
            value_type::SYMBOL => Ok(Symbol::to_name(value)),
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvert {
                from: value.type_name().to_string(),
                to: "string".to_string(),
                help: Some("only int, float, bigint, string, bool, none and symbol can be converted to string".to_string()),
                info: None,
            })),
        }
    }

    pub fn to_string(value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        let vtype = value.decode_type();
        if vtype == value_type::STRING { return Ok(value); }
        let s = Self::to_primitive_string(value, vtype, memory)?;
        Ok(memory.alloc_string(&s))
    }

    pub fn to_display_string(value: DinoRef, memory: &MemoryManager) -> Result<String, RuntimeError> {
        match value.decode_type() {
            value_type::INT => Ok(value.as_int().to_string()),
            value_type::FLOAT => Ok(value.as_float().to_string()),
            value_type::BIGINT => Ok(memory.get_bigint(value.as_bigint()).to_string()),
            value_type::STRING => Ok(memory.get_string(value.decode_index()).to_string()),
            value_type::BOOL => Ok(value.as_bool().to_string()),
            value_type::NONE => Ok("none".to_string()),
            value_type::SYMBOL => Ok(Symbol::to_name(value)),
            value_type::ARRAY => dinojson(value, memory),
            value_type::OBJECT => dinojson(value, memory),
            value_type::FUNCTION => {
                let id = value.get_function_id();
                if value.is_native_fn() {
                    Ok(format!("[NativeFn:{}]", id))
                } else {
                    Ok(format!("[UserFn:{}]", id))
                }
            },
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvert {
                from: value.type_name().to_string(),
                to: "output".to_string(),
                help: None,
                info: None,
            })),
        }
    }

    pub fn to_key_string(value: DinoRef, memory: &MemoryManager) -> Result<String, RuntimeError> {
        match value.decode_type() {
            value_type::INT => Ok(value.as_int().to_string()),
            value_type::FLOAT => Ok(value.as_float().to_string()),
            value_type::BIGINT => Ok(memory.get_bigint(value.as_bigint()).to_string()),
            value_type::STRING => Ok(memory.get_string(value.decode_index()).to_string()),
            value_type::BOOL => Ok(value.as_bool().to_string()),
            value_type::NONE => Ok("none".to_string()),
            value_type::SYMBOL => Ok(Symbol::to_name(value)),
            value_type::ARRAY => Ok(format!("[Array:{}]", value.decode_index())),
            value_type::OBJECT => {
                let id = value.get_object_id();
                if value.is_class() {
                    Ok(format!("[Class:{}]", id))
                } else {
                    Ok(format!("[Object:{}]", id))
                }
            },
            value_type::FUNCTION => {
                let id = value.get_function_id();
                if value.is_native_fn() {
                    Ok(format!("[NativeFn:{}]", id))
                } else {
                    Ok(format!("[UserFn:{}]", id))
                }
            },
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvert {
                from: value.type_name().to_string(),
                to: "key".to_string(),
                help: None,
                info: None,
            })),
        }
    }
}
