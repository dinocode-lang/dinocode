// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/type_conversions.rs
//  Desc:       Type conversion utilities.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    errors::{RuntimeError, RuntimeErrorType},
    utils::{bigint::string_to_bigint_bits, dinojson},
    types::{DinoRef, value_type, Symbol},
};

#[inline(always)]
pub fn bytes_to_i64(bytes: &[u8]) -> Option<i64> {
    if bytes.is_empty() {
        return None;
    }
    
    let mut is_negative = false;
    let mut start = 0;
    
    if bytes[0] == b'-' {
        is_negative = true;
        start = 1;
    } else if bytes[0] == b'+' {
        start = 1;
    }
    
    if start >= bytes.len() {
        return None;
    }
    
    let mut result: i64 = 0;
    
    for &byte in &bytes[start..] {
        if byte < b'0' || byte > b'9' {
            return None;
        }
        let digit = (byte - b'0') as i64;
        
        if result > (i64::MAX - digit) / 10 {
            return None;
        }
        
        result = result * 10 + digit;
    }
    
    if is_negative {
        Some(-result)
    } else {
        Some(result)
    }
}

#[inline(always)]
pub fn bytes_to_f64(bytes: &[u8]) -> Option<f64> {
    // Can be optimized in the future for direct ASCII_bytes-to-float
    let s = std::str::from_utf8(bytes).ok()?;
    s.parse::<f64>().ok()
}

pub struct TypeConverter;

fn value_to_bigint_string(value: DinoRef, memory: &mut MemoryManager) -> Result<String, RuntimeError> {
    match value.decode_type() {
        value_type::BIGINT => {
            let offset = value.decode_index();
            Ok(memory.get_bigint_string(offset))
        },
        value_type::INT => Ok(value.as_int().to_string()),
        value_type::FLOAT => {
            let f = TypeConverter::ensure_finite(value.as_float())?;
            if f.fract() != 0.0 {
                return Err(RuntimeError::Typed(RuntimeErrorType::FloatFractionalToBigInt));
            }
            Ok((f as i64).to_string())
        },
        value_type::STRING => {
            let offset = value.decode_index();
            let s = memory.get_string(offset);
            Ok(s.to_string())
        },
        value_type::BOOL => Ok(if value.as_bool() { "1".to_string() } else { "0".to_string() }),
        _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvertToBigInt(value.type_name().to_string()))),
    }
}

impl TypeConverter {
    #[inline(always)]
    fn ensure_finite(value: f64) -> Result<f64, RuntimeError> {
        if !DinoRef::is_valid_float(value) {
            if value.is_nan() { return Err(RuntimeError::Typed(RuntimeErrorType::ValueIsNaN)); }
            return Err(RuntimeError::Typed(RuntimeErrorType::ValueIsInfinity));
        }
        Ok(value)
    }

    pub fn to_number(value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        match value.decode_type() {
            value_type::INT => Ok(value),
            value_type::FLOAT => Ok(value),
            value_type::BIGINT => {
                let offset = value.decode_index();
                let bigint_str = memory.get_bigint_string(offset);
                match bigint_str.parse::<f64>() {
                    Ok(f) => Ok(DinoRef::float(Self::ensure_finite(f)?)),
                    Err(_) => Err(RuntimeError::Typed(RuntimeErrorType::BigIntToNumberFailed)),
                }
            }
            value_type::STRING => {
                let offset = value.decode_index();
                let string_val = memory.get_string(offset);

                match string_val.parse::<i64>() {
                    Ok(i) if DinoRef::is_valid_int(i) => Ok(DinoRef::int(i)),
                    _ => {
                        match string_val.parse::<f64>() {
                            Ok(f) => Ok(DinoRef::float(Self::ensure_finite(f)?)),
                            Err(_) => Err(RuntimeError::Typed(RuntimeErrorType::StringNotNumeric(string_val.to_string()))),
                        }
                    }
                }
            }
            value_type::BOOL => Ok(if value.as_bool() { DinoRef::ONE } else { DinoRef::ZERO }),
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvertToNumber(value.type_name().to_string()))),
        }
    }

    pub fn to_primitive_int(value: DinoRef, vtype: u16, memory: &mut MemoryManager) -> Result<i64, RuntimeError> {
        match vtype {
            value_type::INT => Ok(value.as_int()),
            value_type::FLOAT => {
                let f = value.as_finite_float()?;
                Ok(f as i64)
            }
            value_type::BIGINT => {
                let offset = value.decode_index();
                let bigint_str = memory.get_bigint_string(offset);
                match bigint_str.parse::<i64>() {
                    Ok(i) if DinoRef::is_valid_int(i) => Ok(i),
                    _ => Err(RuntimeError::Typed(RuntimeErrorType::BigIntTooLargeForInt)),
                }
            }
            value_type::STRING => {
                let offset = value.decode_index();
                let string_val = memory.get_string(offset);
                match string_val.parse::<i64>() {
                    Ok(i) if DinoRef::is_valid_int(i) => Ok(i),
                    _ => Err(RuntimeError::Typed(RuntimeErrorType::StringToIntFailed(string_val.to_string()))),
                }
            }
            value_type::BOOL => Ok(if value.as_bool() { 1 } else { 0 }),
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvertToInt(value.type_name().to_string()))),
        }
    }

    pub fn to_int(value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        let vtype = value.decode_type();
        if vtype == value_type::INT { return Ok(value); }
        let i = Self::to_primitive_int(value, vtype, memory)?;
        Ok(DinoRef::int(i))
    }

    pub fn to_primitive_bool(value: DinoRef, vtype: u16, memory: &mut MemoryManager) -> Result<bool, RuntimeError> {
        match vtype {
            value_type::BOOL => Ok(value.as_bool()),
            value_type::NONE => Ok(false),
            value_type::INT => Ok(value.as_int() != 0),
            value_type::FLOAT => {
                if value.is_nan() {
                    return Err(RuntimeError::Typed(RuntimeErrorType::ValueIsNaN));
                }
                Ok(value.as_float() != 0.0)
            }
            value_type::BIGINT => {
                let bytes = memory.get_const_bytes(value.as_bigint());
                Ok(bytes.len() > 1 && bytes[1..].iter().any(|&b| b != 0))
            }
            value_type::STRING => {
                let offset = value.decode_index();
                Ok(memory.get_const_len(offset) > 0)
            }
            value_type::ARRAY => {
                let offset = value.decode_index();
                Ok(memory.get_array_len(offset) > 0)
            }
            value_type::OBJECT => {
                let object_len = memory.get_object_len(value.get_object_id());
                Ok(object_len > 0)
            }
            _ => Ok(true),
        }
    }

    pub fn to_bool(value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        let vtype = value.decode_type();
        if vtype == value_type::BOOL { return Ok(value); }
        let b = Self::to_primitive_bool(value, vtype, memory)?;
        Ok(if b { DinoRef::TRUE } else { DinoRef::FALSE })
    }

    pub fn to_primitive_float(value: DinoRef, vtype: u16, _memory: &mut MemoryManager) -> Result<f64, RuntimeError> {
        match vtype {
            value_type::FLOAT => value.as_finite_float(),
            value_type::INT => Ok(value.as_int() as f64),
            value_type::BOOL => Ok(if value.as_bool() { 1.0 } else { 0.0 }),
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvertToNumber(value.type_name().to_string()))),
        }
    }

    pub fn to_float(value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        let vtype = value.decode_type();
        if vtype == value_type::FLOAT {
            value.as_finite_float()?;
            return Ok(value);
        }
        let f = Self::to_primitive_float(value, vtype, memory)?;
        Ok(DinoRef::float(f))
    }

    pub fn to_primitive_string(value: DinoRef, vtype: u16, memory: &mut MemoryManager) -> Result<String, RuntimeError> {
        match vtype {
            value_type::STRING => Ok(memory.get_string(value.decode_index()).to_string()),
            value_type::INT => Ok(value.as_int().to_string()),
            value_type::FLOAT => Ok(value.as_float().to_string()),
            value_type::BOOL => Ok(value.as_bool().to_string()),
            value_type::BIGINT => Ok(memory.get_bigint_string(value.decode_index())),
            value_type::NONE => Ok("none".to_string()),
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvertToString(value.type_name().to_string()))),
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
            value_type::BIGINT => Ok(memory.get_bigint_string(value.as_bigint())),
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
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvertToString(value.type_name().to_string()))),
        }
    }

    pub fn to_key_string(value: DinoRef, memory: &MemoryManager) -> Result<String, RuntimeError> {
        match value.decode_type() {
            value_type::INT => Ok(value.as_int().to_string()),
            value_type::FLOAT => Ok(value.as_float().to_string()),
            value_type::BIGINT => Ok(memory.get_bigint_string(value.as_bigint())),
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
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvertToString(value.type_name().to_string()))),
        }
    }

    pub fn to_bigint(value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        let bigint_str = value_to_bigint_string(value, memory)?;
        let bigint_bits = string_to_bigint_bits(&bigint_str).map_err(|e| RuntimeError::TypeError(e))?;
        Ok(memory.alloc_bigint_from_bits(&bigint_bits))
    }
}

