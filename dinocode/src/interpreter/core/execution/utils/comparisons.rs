// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/interpreter/core/execution/utils/comparisons.rs
//  Desc:       Comparison operations.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    types::{DinoRef, value_type},
    memory::MemoryManager,
    utils::parsers::numeric::{Number, parse},
    errors::{RuntimeError, RuntimeErrorType},
};

macro_rules! string_cmp_number {
    ($string:expr, $numeric:expr, $memory:expr, $empty_result:expr, $int_cmp:expr, $float_cmp:expr) => {
        {
            let bytes = $memory.get_const_bytes($string.decode_index());
            if bytes.is_empty() {
                return Ok($empty_result);
            }
            match parse::<Number>(bytes) {
                Ok(Number::Int(num)) => return Ok($int_cmp(num, $numeric)),
                Ok(Number::Float(num)) => return Ok($float_cmp(num, $numeric)),
                Err(_) => return Ok(false),
            }
        }
    };
}

#[inline(always)]
pub fn dyn_equal(a: DinoRef, b: DinoRef, a_type: u16, b_type: u16, memory: &mut MemoryManager) -> Result<bool, RuntimeError> {
    match (a_type, b_type) {
        // Numbers
        (value_type::INT, value_type::FLOAT) => {
            return Ok((a.as_int() as f64) == b.as_float());
        },
        (value_type::FLOAT, value_type::INT) => {
            return Ok(a.as_float() == (b.as_int() as f64));
        },
        // Bool with number
        (value_type::BOOL, value_type::INT) => {
            return Ok((if a.as_bool() { 1.0 } else { 0.0 }) == (b.as_int() as f64));
        },
        (value_type::BOOL, value_type::FLOAT) => {
            return Ok((if a.as_bool() { 1.0 } else { 0.0 }) == b.as_float());
        },
        (value_type::INT, value_type::BOOL) => {
            return Ok((a.as_int() as f64) == (if b.as_bool() { 1.0 } else { 0.0 }));
        },
        (value_type::FLOAT, value_type::BOOL) => {
            return Ok(a.as_float() == (if b.as_bool() { 1.0 } else { 0.0 }));
        },
        // String with number
        (value_type::STRING, value_type::INT) => {
            string_cmp_number!(a, b.as_int(), memory, false,
                |x: i64, y: i64| x == y,
                |x: f64, y: i64| x == (y as f64)
            )
        },
        (value_type::STRING, value_type::FLOAT) => {
            string_cmp_number!(a, b.as_float(), memory, false,
                |x: i64, y: f64| x as f64 == y,
                |x: f64, y: f64| x == y
            )
        },
        (value_type::INT, value_type::STRING) => {
            string_cmp_number!(b, a.as_int(), memory, false,
                |x: i64, y: i64| y == x,
                |x: f64, y: i64| y as f64 == x
            )
        },
        (value_type::FLOAT, value_type::STRING) => {
            string_cmp_number!(b, a.as_float(), memory, false,
                |x: i64, y: f64| y == x as f64,
                |x: f64, y: f64| y == x
            )
        },
        (value_type::OBJECT, _) | (_, value_type::OBJECT) => {
            return Ok(false);
        },
        (value_type::ARRAY, _) | (_, value_type::ARRAY) => {
            return Ok(false);
        },
        (value_type::FUNCTION, _) | (_, value_type::FUNCTION) => {
            return Ok(false);
        },
        _ => {},
    }
    
    Err(RuntimeError::Typed(RuntimeErrorType::InvalidBinaryOperation {
        left: a.type_name().to_string(),
        op: "==".to_string(),
        right: b.type_name().to_string(),
    }))
}

#[inline(always)]
pub fn dyn_greater(a: DinoRef, b: DinoRef, a_type: u16, b_type: u16, memory: &mut MemoryManager) -> Result<bool, RuntimeError> {
    match (a_type, b_type) {
        // Numbers
        (value_type::INT, value_type::FLOAT) => {
            return Ok((a.as_int() as f64) > b.as_float());
        },
        (value_type::FLOAT, value_type::INT) => {
            return Ok(a.as_float() > (b.as_int() as f64));
        },
        // Bool with number
        (value_type::BOOL, value_type::INT) => {
            return Ok((if a.as_bool() { 1.0 } else { 0.0 }) > (b.as_int() as f64));
        },
        (value_type::BOOL, value_type::FLOAT) => {
            return Ok((if a.as_bool() { 1.0 } else { 0.0 }) > b.as_float());
        },
        (value_type::INT, value_type::BOOL) => {
            return Ok((a.as_int() as f64) > (if b.as_bool() { 1.0 } else { 0.0 }));
        },
        (value_type::FLOAT, value_type::BOOL) => {
            return Ok(a.as_float() > (if b.as_bool() { 1.0 } else { 0.0 }));
        },
        // String with number
        (value_type::STRING, value_type::INT) => {
            string_cmp_number!(a, b.as_int(), memory, false,
                |x: i64, y: i64| x > y,
                |x: f64, y: i64| x > y as f64
            )
        },
        (value_type::STRING, value_type::FLOAT) => {
            string_cmp_number!(a, b.as_float(), memory, false,
                |x: i64, y: f64| x as f64 > y,
                |x: f64, y: f64| x > y
            )
        },
        (value_type::INT, value_type::STRING) => {
            string_cmp_number!(b, a.as_int(), memory, true,
                |x: i64, y: i64| y > x,
                |x: f64, y: i64| y as f64 > x
            )
        },
        (value_type::FLOAT, value_type::STRING) => {
            string_cmp_number!(b, a.as_float(), memory, true,
                |x: i64, y: f64| y > x as f64,
                |x: f64, y: f64| y > x
            )
        },
        _ => {},
    }
    
    Err(RuntimeError::Typed(RuntimeErrorType::InvalidBinaryOperation {
        left: a.type_name().to_string(),
        op: ">".to_string(),
        right: b.type_name().to_string(),
    }))
}

#[inline(always)]
pub fn dyn_less(a: DinoRef, b: DinoRef, a_type: u16, b_type: u16, memory: &mut MemoryManager) -> Result<bool, RuntimeError> {
    match (a_type, b_type) {
        // Numbers
        (value_type::INT, value_type::FLOAT) => {
            return Ok((a.as_int() as f64) < b.as_float());
        },
        (value_type::FLOAT, value_type::INT) => {
            return Ok(a.as_float() < (b.as_int() as f64));
        },
        // Bool with number
        (value_type::BOOL, value_type::INT) => {
            return Ok((if a.as_bool() { 1.0 } else { 0.0 }) < (b.as_int() as f64));
        },
        (value_type::BOOL, value_type::FLOAT) => {
            return Ok((if a.as_bool() { 1.0 } else { 0.0 }) < b.as_float());
        },
        (value_type::INT, value_type::BOOL) => {
            return Ok((a.as_int() as f64) < (if b.as_bool() { 1.0 } else { 0.0 }));
        },
        (value_type::FLOAT, value_type::BOOL) => {
            return Ok(a.as_float() < (if b.as_bool() { 1.0 } else { 0.0 }));
        },
        // String with number
        (value_type::STRING, value_type::INT) => {
            string_cmp_number!(a, b.as_int(), memory, false,
                |x: i64, y: i64| x < y,
                |x: f64, y: i64| x < y as f64
            )
        },
        (value_type::STRING, value_type::FLOAT) => {
            string_cmp_number!(a, b.as_float(), memory, false,
                |x: i64, y: f64| (x as f64) < y,
                |x: f64, y: f64| x < y
            )
        },
        (value_type::INT, value_type::STRING) => {
            string_cmp_number!(b, a.as_int(), memory, false,
                |x: i64, y: i64| y < x,
                |x: f64, y: i64| (y as f64) < x
            )
        },
        (value_type::FLOAT, value_type::STRING) => {
            string_cmp_number!(b, a.as_float(), memory, false,
                |x: i64, y: f64| y < x as f64,
                |x: f64, y: f64| y < x
            )
        },
        _ => {},
    }
    
    Err(RuntimeError::Typed(RuntimeErrorType::InvalidBinaryOperation {
        left: a.type_name().to_string(),
        op: "<".to_string(),
        right: b.type_name().to_string(),
    }))
}
