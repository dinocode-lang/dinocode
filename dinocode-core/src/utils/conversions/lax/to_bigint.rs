// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/conversions/lax/to_bigint.rs
//  Desc:       Lax BigInt type conversions for TypeConverter
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
        parsers::numeric::parse_lax,
    },
    builtins::bigint::view::BigIntView,
};
use super::super::TypeConverter;

impl TypeConverter {
    pub fn to_bigint_lax(value: DinoRef, base: Option<u32>, memory: &mut MemoryManager) -> Result<DinoRef, RuntimeError> {
        match value.decode_type() {
            value_type::BIGINT => return Ok(value),
            value_type::INT => {
                let bigint = BigInt::from_i64(value.as_int());
                return Ok(BigIntView::new(&bigint, memory).as_ref());
            }
            value_type::FLOAT => {
                let bigint = BigInt::from_f64(value.as_finite_float()?);
                return Ok(BigIntView::new(&bigint, memory).as_ref());
            }
            value_type::STRING => {
                let offset = value.as_index();
                let bytes = memory.arena.get_bytes(offset);
                let bigint = parse_lax::<BigInt>(bytes, base).map_err(|e| RuntimeError::NumericParse(e))?;
                return Ok(BigIntView::new(&bigint, memory).as_ref());
            }
            value_type::BOOL => {
                let bigint = if value.as_bool() { BigInt::ONE } else { BigInt::ZERO };
                return Ok(BigIntView::new(&bigint, memory).as_ref());
            }
            _ => {}
        }
        Err(RuntimeError::CannotConvert {
            from: value.type_name(),
            to: "bigint",
            info: Some("only int, float, string, and bool can be converted to BigInt"),
            help: None,
        })
    }
}
