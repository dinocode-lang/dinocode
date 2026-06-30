// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/conversions/strict/to_number.rs
//  Desc:       Strict numeric type conversions for TypeConverter
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    errors::{RuntimeError, RuntimeErrorType},
    types::{DinoRef, value_type},
    utils::{
        bigint::BigInt,
        parsers::numeric::{
            parse,
            Number,
            is_valid_int,
            error_i64,
        },
    },
};
use super::super::TypeConverter;

impl TypeConverter {
    pub fn to_primitive_int(value: DinoRef, vtype: u16, memory: &mut MemoryManager) -> Result<i64, RuntimeError> {
        match vtype {
            value_type::INT => Ok(value.as_int()),
            value_type::FLOAT => {
                Ok(value.as_finite_float()? as i64)
            }
            value_type::BIGINT => {
                let offset = value.decode_index();
                let bytes = memory.get_const_bytes(offset);
                let bigint = BigInt::from_slice(bytes);
                bigint.to_i64()
                    .map_err(|e| RuntimeError::Typed(RuntimeErrorType::NumericParse(e)))
                    .and_then(|i| {
                        if is_valid_int(i) { Ok(i) }
                        else { Err(RuntimeError::Typed(RuntimeErrorType::NumericParse(error_i64(&bytes)))) }
                    })
            }
            value_type::STRING => {
                let offset = value.decode_index();
                let bytes = memory.get_const_bytes(offset);
                parse::<i64>(bytes).map_err(|e| RuntimeError::Typed(RuntimeErrorType::NumericParse(e)))
            }
            value_type::BOOL => Ok(
                if value.as_bool() { 1 } else { 0 }
            ),
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvert {
                from: value.type_name().to_string(),
                to: "int".to_string(),
                info: Some("only float, bigint, string, and bool can be converted to int".to_string()),
                help: None,
            })),
        }
    }

    pub fn to_int(value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        let vtype = value.decode_type();
        if vtype == value_type::INT { return Ok(value); }
        let i = Self::to_primitive_int(value, vtype, memory)?;
        Ok(DinoRef::int(i))
    }

    pub fn to_primitive_float(value: DinoRef, vtype: u16, memory: &mut MemoryManager) -> Result<f64, RuntimeError> {
        match vtype {
            value_type::INT => Ok(value.as_int() as f64),
            value_type::FLOAT => value.as_finite_float(),
            value_type::BIGINT => {
                let offset = value.decode_index();
                let bytes = memory.get_const_bytes(offset);
                let bigint = BigInt::from_slice(bytes);
                bigint.to_f64()
                    .map_err(|e| RuntimeError::Typed(RuntimeErrorType::NumericParse(e)))
            }
            value_type::STRING => {
                let offset = value.decode_index();
                let bytes = memory.get_const_bytes(offset);
                parse::<f64>(bytes).map_err(|e| RuntimeError::Typed(RuntimeErrorType::NumericParse(e)))
            }
            value_type::BOOL => Ok(
                if value.as_bool() { 1.0 } else { 0.0 }
            ),
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvert {
                from: value.type_name().to_string(),
                to: "float".to_string(),
                info: Some("only int, bigint, string, and bool can be converted to float".to_string()),
                help: None,
            })),
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

    pub fn to_number(value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        match value.decode_type() {
            value_type::INT => Ok(value),
            value_type::FLOAT => Ok(value),
            value_type::BIGINT => {
                let offset = value.decode_index();
                let bytes = memory.get_const_bytes(offset);
                let bigint = BigInt::from_slice(bytes);
                bigint.to_f64()
                    .map(|f| DinoRef::float(f))
                    .map_err(|e| RuntimeError::Typed(RuntimeErrorType::NumericParse(e)))
            }
            value_type::STRING => {
                let offset = value.decode_index();
                let bytes = memory.get_const_bytes(offset);

                match parse::<Number>(bytes) {
                    Ok(Number::Int(i)) => Ok(DinoRef::int(i)),
                    Ok(Number::Float(f)) => Ok(DinoRef::float(f)),
                    Err(e) => Err(RuntimeError::Typed(RuntimeErrorType::NumericParse(e))),
                }
            }
            value_type::BOOL => Ok(
                if value.as_bool() { DinoRef::ONE } else { DinoRef::ZERO }
            ),
            _ => Err(RuntimeError::Typed(RuntimeErrorType::CannotConvert {
                from: value.type_name().to_string(),
                to: "number".to_string(),
                info: Some("only int, float, bigint, string, and bool can be converted to number".to_string()),
                help: None,
            })),
        }
    }
}
