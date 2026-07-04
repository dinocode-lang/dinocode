// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/conversions/lax/to_number.rs
//  Desc:       Lax numeric type conversions for TypeConverter
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    errors::RuntimeError,
    types::{DinoRef, value_type},
    utils::{
        bigint::BigInt,
        parsers::numeric::{
            parse_lax,
            Number,
            is_valid_int,
            error_i64,
        },
    },
};
use super::super::TypeConverter;

impl TypeConverter {
    pub fn to_primitive_int_lax(value: DinoRef, vtype: u16, base: Option<u32>, memory: &mut MemoryManager) -> Result<i64, RuntimeError> {
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
                    .map_err(|e| RuntimeError::NumericParse(e))
                    .and_then(|i| {
                        if is_valid_int(i) { Ok(i) }
                        else { Err(RuntimeError::NumericParse(error_i64(&bytes))) }
                    })
            }
            value_type::STRING => {
                let offset = value.decode_index();
                let bytes = memory.get_const_bytes(offset);
                parse_lax::<i64>(bytes, base).map_err(|e| RuntimeError::NumericParse(e))
            }
            value_type::BOOL => Ok(
                if value.as_bool() { 1 } else { 0 }
            ),
            _ => Err(RuntimeError::CannotConvert {
                from: value.type_name(),
                to: "int",
                info: Some("only float, bigint, string, and bool can be converted to int"),
                help: None,
            }),
        }
    }

    pub fn to_int_lax(value: DinoRef, base: Option<u32>, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        let vtype = value.decode_type();
        if vtype == value_type::INT { return Ok(value); }
        let i = Self::to_primitive_int_lax(value, vtype, base, memory)?;
        Ok(DinoRef::int(i))
    }

    pub fn to_primitive_float_lax(value: DinoRef, vtype: u16, memory: &mut MemoryManager) -> Result<f64, RuntimeError> {
        match vtype {
            value_type::INT => Ok(value.as_int() as f64),
            value_type::FLOAT => value.as_finite_float(),
            value_type::BIGINT => {
                let offset = value.decode_index();
                let bytes = memory.get_const_bytes(offset);
                let bigint = BigInt::from_slice(bytes);
                bigint.to_f64().map_err(|e| RuntimeError::NumericParse(e))
            }
            value_type::STRING => {
                let offset = value.decode_index();
                let bytes = memory.get_const_bytes(offset);
                parse_lax::<f64>(bytes, None).map_err(|e| RuntimeError::NumericParse(e))
            }
            value_type::BOOL => Ok(
                if value.as_bool() { 1.0 } else { 0.0 }
            ),
            _ => Err(RuntimeError::CannotConvert {
                from: value.type_name(),
                to: "float",
                info: Some("only int, bigint, string, and bool can be converted to float"),
                help: None,
            }),
        }
    }

    pub fn to_float_lax(value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        let vtype = value.decode_type();
        if vtype == value_type::FLOAT {
            value.as_finite_float()?;
            return Ok(value);
        }
        let f = Self::to_primitive_float_lax(value, vtype, memory)?;
        Ok(DinoRef::float(f))
    }

    pub fn to_number_lax(value: DinoRef, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        match value.decode_type() {
            value_type::INT => Ok(value),
            value_type::FLOAT => Ok(value),
            value_type::BIGINT => {
                let offset = value.decode_index();
                let bytes = memory.get_const_bytes(offset);
                let bigint = BigInt::from_slice(bytes);
                bigint.to_f64()
                    .map(|f| DinoRef::float(f))
                    .map_err(|e| RuntimeError::NumericParse(e))
            }
            value_type::STRING => {
                let offset = value.decode_index();
                let bytes = memory.get_const_bytes(offset);

                match parse_lax::<Number>(bytes, None) {
                    Ok(Number::Int(i)) => Ok(DinoRef::int(i)),
                    Ok(Number::Float(f)) => Ok(DinoRef::float(f)),
                    Err(e) => Err(RuntimeError::NumericParse(e)),
                }
            }
            value_type::BOOL => Ok(
                if value.as_bool() { DinoRef::ONE } else { DinoRef::ZERO }
            ),
            _ => Err(RuntimeError::CannotConvert {
                from: value.type_name(),
                to: "number",
                info: Some("only int, float, bigint, string, and bool can be converted to number"),
                help: None,
            }),
        }
    }
}
