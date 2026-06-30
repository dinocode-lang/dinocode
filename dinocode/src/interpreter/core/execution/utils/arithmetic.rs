// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/interpreter/core/execution/utils/arithmetic.rs
//  Desc:       Arithmetic operations.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    types::{DinoRef, value_type},
    memory::MemoryManager,
    utils::parsers::numeric::{Number, parse},
    errors::{RuntimeError, RuntimeErrorType, Result},
};

macro_rules! string_op_number {
    ($string:expr, $numeric:expr, $memory:expr, $string_left_op:expr, $string_right_op:expr) => {
        {
            let bytes = $memory.get_const_bytes($string.decode_index());
            match parse::<Number>(bytes) {
                Ok(Number::Int(si)) => $string_left_op(si, $numeric),
                Ok(Number::Float(sf)) => $string_right_op(sf, $numeric),
                Err(e) => Err(RuntimeError::Typed(RuntimeErrorType::NumericParse(e))),
            }
        }
    };
}

macro_rules! string_op_string {
    ($a:expr, $b:expr, $memory:expr, $int_op:expr, $float_op:expr) => {
        {
            let a_bytes = $memory.get_const_bytes($a.decode_index());
            let b_bytes = $memory.get_const_bytes($b.decode_index());
            
            let a_num = parse::<Number>(a_bytes);
            let b_num = parse::<Number>(b_bytes);
            
            match (a_num, b_num) {
                (Ok(Number::Int(ai)), Ok(Number::Int(bi))) => $int_op(ai, bi),
                (Ok(Number::Int(ai)), Ok(Number::Float(bf))) => $float_op(ai as f64, bf),
                (Ok(Number::Float(af)), Ok(Number::Int(bi))) => $float_op(af, bi as f64),
                (Ok(Number::Float(af)), Ok(Number::Float(bf))) => $float_op(af, bf),
                (Ok(_), Err(e)) => Err(RuntimeError::Typed(RuntimeErrorType::NumericParse(e))),
                (Err(e), Ok(_)) => Err(RuntimeError::Typed(RuntimeErrorType::NumericParse(e))),
                (Err(e), _) => Err(RuntimeError::Typed(RuntimeErrorType::NumericParse(e))),
            }
        }
    };
}

#[inline(always)]
pub fn dyn_add(a: DinoRef, b: DinoRef, a_type: u16, b_type: u16, memory: &mut MemoryManager) -> Result<DinoRef> {
    match (a_type, b_type) {
        (value_type::INT, value_type::FLOAT) => {
            Ok(DinoRef::float(a.as_int() as f64 + b.as_float()))
        },
        (value_type::FLOAT, value_type::INT) => {
            Ok(DinoRef::float(a.as_float() + b.as_int() as f64))
        },
        (value_type::BOOL, value_type::INT) => {
            let a_val = if a.as_bool() { 1i64 } else { 0 };
            Ok(DinoRef::int(a_val.wrapping_add(b.as_int())))
        },
        (value_type::INT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1i64 } else { 0 };
            Ok(DinoRef::int(a.as_int().wrapping_add(b_val)))
        },
        (value_type::BOOL, value_type::FLOAT) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a_val + b.as_float()))
        },
        (value_type::FLOAT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a.as_float() + b_val))
        },
        (value_type::STRING, value_type::INT) => {
            string_op_number!(a, b.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::int(x.wrapping_add(y))),
                |x: f64, y: i64| Ok(DinoRef::float(x + y as f64))
            )
        },
        (value_type::INT, value_type::STRING) => {
            string_op_number!(b, a.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::int(y.wrapping_add(x))),
                |x: f64, y: i64| Ok(DinoRef::float(y as f64 + x))
            )
        },
        (value_type::STRING, value_type::FLOAT) => {
            string_op_number!(a, b.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float(x as f64 + y)),
                |x: f64, y: f64| Ok(DinoRef::float(x + y))
            )
        },
        (value_type::FLOAT, value_type::STRING) => {
            string_op_number!(b, a.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float(y + x as f64)),
                |x: f64, y: f64| Ok(DinoRef::float(y + x))
            )
        },
        (value_type::STRING, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1i64 } else { 0 };
            string_op_number!(a, b_val, memory,
                |x: i64, y: i64| Ok(DinoRef::int(x.wrapping_add(y))),
                |x: f64, y: i64| Ok(DinoRef::float(x + y as f64))
            )
        },
        (value_type::BOOL, value_type::STRING) => {
            let a_val = if a.as_bool() { 1i64 } else { 0 };
            string_op_number!(b, a_val, memory,
                |x: i64, y: i64| Ok(DinoRef::int(y.wrapping_add(x))),
                |x: f64, y: i64| Ok(DinoRef::float(y as f64 + x))
            )
        },
        (value_type::STRING, value_type::STRING) => {
            string_op_string!(a, b, memory,
                |ai: i64, bi: i64| Ok(DinoRef::int(ai.wrapping_add(bi))),
                |af: f64, bf: f64| Ok(DinoRef::float(af + bf))
            )
        },
        (value_type::BOOL, value_type::BOOL) => {
            let a_val = if a.as_bool() { 1i64 } else { 0 };
            let b_val = if b.as_bool() { 1i64 } else { 0 };
            Ok(DinoRef::int(a_val.wrapping_add(b_val)))
        },
        _ => Err(RuntimeError::Typed(RuntimeErrorType::InvalidBinaryOperation {
            left: a.type_name().to_string(),
            op: "+".to_string(),
            right: b.type_name().to_string(),
        })),
    }
}

#[inline(always)]
pub fn dyn_sub(a: DinoRef, b: DinoRef, a_type: u16, b_type: u16, memory: &mut MemoryManager) -> Result<DinoRef> {
    match (a_type, b_type) {
        (value_type::INT, value_type::FLOAT) => {
            Ok(DinoRef::float(a.as_int() as f64 - b.as_float()))
        },
        (value_type::FLOAT, value_type::INT) => {
            Ok(DinoRef::float(a.as_float() - b.as_int() as f64))
        },
        (value_type::BOOL, value_type::INT) => {
            let a_val = if a.as_bool() { 1i64 } else { 0 };
            Ok(DinoRef::int(a_val.wrapping_sub(b.as_int())))
        },
        (value_type::INT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1i64 } else { 0 };
            Ok(DinoRef::int(a.as_int().wrapping_sub(b_val)))
        },
        (value_type::BOOL, value_type::FLOAT) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a_val - b.as_float()))
        },
        (value_type::FLOAT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a.as_float() - b_val))
        },
        (value_type::STRING, value_type::INT) => {
            string_op_number!(a, b.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::int(x.wrapping_sub(y))),
                |x: f64, y: i64| Ok(DinoRef::float(x - y as f64))
            )
        },
        (value_type::INT, value_type::STRING) => {
            string_op_number!(b, a.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::int(y.wrapping_sub(x))),
                |x: f64, y: i64| Ok(DinoRef::float(y as f64 - x))
            )
        },
        (value_type::STRING, value_type::FLOAT) => {
            string_op_number!(a, b.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float(x as f64 - y)),
                |x: f64, y: f64| Ok(DinoRef::float(x - y))
            )
        },
        (value_type::FLOAT, value_type::STRING) => {
            string_op_number!(b, a.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float(y - x as f64)),
                |x: f64, y: f64| Ok(DinoRef::float(y - x))
            )
        },
        (value_type::STRING, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1i64 } else { 0 };
            string_op_number!(a, b_val, memory,
                |x: i64, y: i64| Ok(DinoRef::int(x.wrapping_sub(y))),
                |x: f64, y: i64| Ok(DinoRef::float(x - y as f64))
            )
        },
        (value_type::BOOL, value_type::STRING) => {
            let a_val = if a.as_bool() { 1i64 } else { 0 };
            string_op_number!(b, a_val, memory,
                |x: i64, y: i64| Ok(DinoRef::int(y.wrapping_sub(x))),
                |x: f64, y: i64| Ok(DinoRef::float(y as f64 - x))
            )
        },
        (value_type::STRING, value_type::STRING) => {
            string_op_string!(a, b, memory,
                |ai: i64, bi: i64| Ok(DinoRef::int(ai.wrapping_sub(bi))),
                |af: f64, bf: f64| Ok(DinoRef::float(af - bf))
            )
        },
        (value_type::BOOL, value_type::BOOL) => {
            let a_val = if a.as_bool() { 1i64 } else { 0 };
            let b_val = if b.as_bool() { 1i64 } else { 0 };
            Ok(DinoRef::int(a_val.wrapping_sub(b_val)))
        },
        _ => Err(RuntimeError::Typed(RuntimeErrorType::InvalidBinaryOperation {
            left: a.type_name().to_string(),
            op: "-".to_string(),
            right: b.type_name().to_string(),
        })),
    }
}

#[inline(always)]
pub fn dyn_mul(a: DinoRef, b: DinoRef, a_type: u16, b_type: u16, memory: &mut MemoryManager) -> Result<DinoRef> {
    match (a_type, b_type) {
        (value_type::INT, value_type::FLOAT) => {
            Ok(DinoRef::float(a.as_int() as f64 * b.as_float()))
        },
        (value_type::FLOAT, value_type::INT) => {
            Ok(DinoRef::float(a.as_float() * b.as_int() as f64))
        },
        (value_type::BOOL, value_type::INT) => {
            let a_val = if a.as_bool() { 1i64 } else { 0 };
            Ok(DinoRef::int(a_val.wrapping_mul(b.as_int())))
        },
        (value_type::INT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1i64 } else { 0 };
            Ok(DinoRef::int(a.as_int().wrapping_mul(b_val)))
        },
        (value_type::BOOL, value_type::FLOAT) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a_val * b.as_float()))
        },
        (value_type::FLOAT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a.as_float() * b_val))
        },
        (value_type::STRING, value_type::INT) => {
            string_op_number!(a, b.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::int(x.wrapping_mul(y))),
                |x: f64, y: i64| Ok(DinoRef::float(x * y as f64))
            )
        },
        (value_type::INT, value_type::STRING) => {
            string_op_number!(b, a.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::int(y.wrapping_mul(x))),
                |x: f64, y: i64| Ok(DinoRef::float(y as f64 * x))
            )
        },
        (value_type::STRING, value_type::FLOAT) => {
            string_op_number!(a, b.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float(x as f64 * y)),
                |x: f64, y: f64| Ok(DinoRef::float(x * y))
            )
        },
        (value_type::FLOAT, value_type::STRING) => {
            string_op_number!(b, a.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float(y * x as f64)),
                |x: f64, y: f64| Ok(DinoRef::float(y * x))
            )
        },
        (value_type::STRING, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1i64 } else { 0 };
            string_op_number!(a, b_val, memory,
                |x: i64, y: i64| Ok(DinoRef::int(x.wrapping_mul(y))),
                |x: f64, y: i64| Ok(DinoRef::float(x * y as f64))
            )
        },
        (value_type::BOOL, value_type::STRING) => {
            let a_val = if a.as_bool() { 1i64 } else { 0 };
            string_op_number!(b, a_val, memory,
                |x: i64, y: i64| Ok(DinoRef::int(y.wrapping_mul(x))),
                |x: f64, y: i64| Ok(DinoRef::float(y as f64 * x))
            )
        },
        (value_type::STRING, value_type::STRING) => {
            string_op_string!(a, b, memory,
                |ai: i64, bi: i64| Ok(DinoRef::int(ai.wrapping_mul(bi))),
                |af: f64, bf: f64| Ok(DinoRef::float(af * bf))
            )
        },
        (value_type::BOOL, value_type::BOOL) => {
            let a_val = if a.as_bool() { 1i64 } else { 0 };
            let b_val = if b.as_bool() { 1i64 } else { 0 };
            Ok(DinoRef::int(a_val.wrapping_mul(b_val)))
        },
        _ => Err(RuntimeError::Typed(RuntimeErrorType::InvalidBinaryOperation {
            left: a.type_name().to_string(),
            op: "*".to_string(),
            right: b.type_name().to_string(),
        })),
    }
}

#[inline(always)]
pub fn dyn_div(a: DinoRef, b: DinoRef, a_type: u16, b_type: u16, memory: &mut MemoryManager) -> Result<DinoRef> {
    match (a_type, b_type) {
        (value_type::INT, value_type::FLOAT) => {
            Ok(DinoRef::float(a.as_int() as f64 / b.as_float()))
        },
        (value_type::FLOAT, value_type::INT) => {
            Ok(DinoRef::float(a.as_float() / b.as_int() as f64))
        },
        (value_type::BOOL, value_type::INT) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a_val / b.as_int() as f64))
        },
        (value_type::INT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a.as_int() as f64 / b_val))
        },
        (value_type::BOOL, value_type::FLOAT) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a_val / b.as_float()))
        },
        (value_type::FLOAT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a.as_float() / b_val))
        },
        (value_type::STRING, value_type::INT) => {
            string_op_number!(a, b.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::float(x as f64 / y as f64)),
                |x: f64, y: i64| Ok(DinoRef::float(x / y as f64))
            )
        },
        (value_type::INT, value_type::STRING) => {
            string_op_number!(b, a.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::float(y as f64 / x as f64)),
                |x: f64, y: i64| Ok(DinoRef::float(y as f64 / x))
            )
        },
        (value_type::STRING, value_type::FLOAT) => {
            string_op_number!(a, b.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float(x as f64 / y)),
                |x: f64, y: f64| Ok(DinoRef::float(x / y))
            )
        },
        (value_type::FLOAT, value_type::STRING) => {
            string_op_number!(b, a.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float(y / x as f64)),
                |x: f64, y: f64| Ok(DinoRef::float(y / x))
            )
        },
        (value_type::STRING, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            string_op_number!(a, b_val, memory,
                |x: i64, y: f64| Ok(DinoRef::float(x as f64 / y)),
                |x: f64, y: f64| Ok(DinoRef::float(x / y))
            )
        },
        (value_type::BOOL, value_type::STRING) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            string_op_number!(b, a_val, memory,
                |x: i64, y: f64| Ok(DinoRef::float(y / x as f64)),
                |x: f64, y: f64| Ok(DinoRef::float(y / x))
            )
        },
        (value_type::STRING, value_type::STRING) => {
            string_op_string!(a, b, memory,
                |ai, bi| Ok(DinoRef::float(ai as f64 / bi as f64)),
                |af, bf| Ok(DinoRef::float(af / bf))
            )
        },
        (value_type::BOOL, value_type::BOOL) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a_val / b_val))
        },
        _ => Err(RuntimeError::Typed(RuntimeErrorType::InvalidBinaryOperation {
            left: a.type_name().to_string(),
            op: "/".to_string(),
            right: b.type_name().to_string(),
        })),
    }
}

#[inline(always)]
pub fn dyn_mod(a: DinoRef, b: DinoRef, a_type: u16, b_type: u16, memory: &mut MemoryManager) -> Result<DinoRef> {
    match (a_type, b_type) {
        (value_type::INT, value_type::FLOAT) => {
            Ok(DinoRef::float(a.as_int() as f64 % b.as_float()))
        },
        (value_type::FLOAT, value_type::INT) => {
            Ok(DinoRef::float(a.as_float() % b.as_int() as f64))
        },
        (value_type::BOOL, value_type::INT) => {
            let a_val = if a.as_bool() { 1i64 } else { 0 };
            Ok(DinoRef::int(a_val % b.as_int()))
        },
        (value_type::INT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1i64 } else { 0 };
            Ok(DinoRef::int(a.as_int() % b_val))
        },
        (value_type::BOOL, value_type::FLOAT) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a_val % b.as_float()))
        },
        (value_type::FLOAT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a.as_float() % b_val))
        },
        (value_type::STRING, value_type::INT) => {
            string_op_number!(a, b.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::int(x % y)),
                |x: f64, y: i64| Ok(DinoRef::float(x % y as f64))
            )
        },
        (value_type::INT, value_type::STRING) => {
            string_op_number!(b, a.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::int(y % x)),
                |x: f64, y: i64| Ok(DinoRef::float(y as f64 % x))
            )
        },
        (value_type::STRING, value_type::FLOAT) => {
            string_op_number!(a, b.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float(x as f64 % y)),
                |x: f64, y: f64| Ok(DinoRef::float(x % y))
            )
        },
        (value_type::FLOAT, value_type::STRING) => {
            string_op_number!(b, a.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float(y % x as f64)),
                |x: f64, y: f64| Ok(DinoRef::float(y % x))
            )
        },
        (value_type::STRING, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1i64 } else { 0 };
            string_op_number!(a, b_val, memory,
                |x: i64, y: i64| Ok(DinoRef::int(x % y)),
                |x: f64, y: i64| Ok(DinoRef::float(x % y as f64))
            )
        },
        (value_type::BOOL, value_type::STRING) => {
            let a_val = if a.as_bool() { 1i64 } else { 0 };
            string_op_number!(b, a_val, memory,
                |x: i64, y: i64| Ok(DinoRef::int(y % x)),
                |x: f64, y: i64| Ok(DinoRef::float(y as f64 % x))
            )
        },
        (value_type::STRING, value_type::STRING) => {
            string_op_string!(a, b, memory,
                |ai: i64, bi: i64| Ok(DinoRef::int(ai % bi)),
                |af: f64, bf: f64| Ok(DinoRef::float(af % bf))
            )
        },
        (value_type::BOOL, value_type::BOOL) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a_val % b_val))
        },
        _ => Err(RuntimeError::Typed(RuntimeErrorType::InvalidBinaryOperation {
            left: a.type_name().to_string(),
            op: "%".to_string(),
            right: b.type_name().to_string(),
        })),
    }
}

#[inline(always)]
pub fn dyn_floor_div(a: DinoRef, b: DinoRef, a_type: u16, b_type: u16, memory: &mut MemoryManager) -> Result<DinoRef> {
    match (a_type, b_type) {
        (value_type::INT, value_type::FLOAT) => {
            Ok(DinoRef::float((a.as_int() as f64 / b.as_float()).floor()))
        },
        (value_type::FLOAT, value_type::INT) => {
            Ok(DinoRef::float((a.as_float() / b.as_int() as f64).floor()))
        },
        (value_type::BOOL, value_type::INT) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float((a_val / b.as_int() as f64).floor()))
        },
        (value_type::INT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float((a.as_int() as f64 / b_val).floor()))
        },
        (value_type::BOOL, value_type::FLOAT) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float((a_val / b.as_float()).floor()))
        },
        (value_type::FLOAT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float((a.as_float() / b_val).floor()))
        },
        (value_type::STRING, value_type::INT) => {
            string_op_number!(a, b.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::float((x as f64 / y as f64).floor())),
                |x: f64, y: i64| Ok(DinoRef::float((x / y as f64).floor()))
            )
        },
        (value_type::INT, value_type::STRING) => {
            string_op_number!(b, a.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::float((y as f64 / x as f64).floor())),
                |x: f64, y: i64| Ok(DinoRef::float((y as f64 / x).floor()))
            )
        },
        (value_type::STRING, value_type::FLOAT) => {
            string_op_number!(a, b.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float((x as f64 / y).floor())),
                |x: f64, y: f64| Ok(DinoRef::float((x / y).floor()))
            )
        },
        (value_type::FLOAT, value_type::STRING) => {
            string_op_number!(b, a.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float((y / x as f64).floor())),
                |x: f64, y: f64| Ok(DinoRef::float((y / x).floor()))
            )
        },
        (value_type::STRING, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            string_op_number!(a, b_val, memory,
                |x: i64, y: f64| Ok(DinoRef::float((x as f64 / y).floor())),
                |x: f64, y: f64| Ok(DinoRef::float((x / y).floor()))
            )
        },
        (value_type::BOOL, value_type::STRING) => {
            let a_val = if a.as_bool() { 1.0 } else { 0.0 };
            string_op_number!(b, a_val, memory,
                |x: i64, y: f64| Ok(DinoRef::float((y / x as f64).floor())),
                |x: f64, y: f64| Ok(DinoRef::float((y / x).floor()))
            )
        },
        (value_type::STRING, value_type::STRING) => {
            string_op_string!(a, b, memory,
                |ai: i64, bi: i64| Ok(DinoRef::float((ai as f64 / bi as f64).floor())),
                |af: f64, bf: f64| Ok(DinoRef::float((af / bf).floor()))
            )
        },
        (value_type::BOOL, value_type::BOOL) => {
            let a_val: f64 = if a.as_bool() { 1.0 } else { 0.0 };
            let b_val: f64 = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float((a_val / b_val).floor()))
        },
        _ => Err(RuntimeError::Typed(RuntimeErrorType::InvalidBinaryOperation {
            left: a.type_name().to_string(),
            op: "//".to_string(),
            right: b.type_name().to_string(),
        })),
    }
}

#[inline(always)]
pub fn dyn_pow(a: DinoRef, b: DinoRef, a_type: u16, b_type: u16, memory: &mut MemoryManager) -> Result<DinoRef> {
    match (a_type, b_type) {
        (value_type::INT, value_type::FLOAT) => {
            Ok(DinoRef::float((a.as_int() as f64).powf(b.as_float())))
        },
        (value_type::FLOAT, value_type::INT) => {
            Ok(DinoRef::float(a.as_float().powf(b.as_int() as f64)))
        },
        (value_type::BOOL, value_type::INT) => {
            let a_val: f64 = if a.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a_val.powf(b.as_int() as f64)))
        },
        (value_type::INT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float((a.as_int() as f64).powf(b_val)))
        },
        (value_type::BOOL, value_type::FLOAT) => {
            let a_val: f64 = if a.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a_val.powf(b.as_float())))
        },
        (value_type::FLOAT, value_type::BOOL) => {
            let b_val = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a.as_float().powf(b_val)))
        },
        (value_type::STRING, value_type::INT) => {
            string_op_number!(a, b.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::float((x as f64).powf(y as f64))),
                |x: f64, y: i64| Ok(DinoRef::float(x.powf(y as f64)))
            )
        },
        (value_type::INT, value_type::STRING) => {
            string_op_number!(b, a.as_int(), memory,
                |x: i64, y: i64| Ok(DinoRef::float((y as f64).powf(x as f64))),
                |x: f64, y: i64| Ok(DinoRef::float((y as f64).powf(x)))
            )
        },
        (value_type::STRING, value_type::FLOAT) => {
            string_op_number!(a, b.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float((x as f64).powf(y))),
                |x: f64, y: f64| Ok(DinoRef::float(x.powf(y)))
            )
        },
        (value_type::FLOAT, value_type::STRING) => {
            string_op_number!(b, a.as_float(), memory,
                |x: i64, y: f64| Ok(DinoRef::float(y.powf(x as f64))),
                |x: f64, y: f64| Ok(DinoRef::float(y.powf(x)))
            )
        },
        (value_type::STRING, value_type::BOOL) => {
            let b_val: f64 = if b.as_bool() { 1.0 } else { 0.0 };
            string_op_number!(a, b_val, memory,
                |x: i64, y: f64| Ok(DinoRef::float((x as f64).powf(y))),
                |x: f64, y: f64| Ok(DinoRef::float(x.powf(y)))
            )
        },
        (value_type::BOOL, value_type::STRING) => {
            let a_val: f64 = if a.as_bool() { 1.0 } else { 0.0 };
            string_op_number!(b, a_val, memory,
                |x: i64, y: f64| Ok(DinoRef::float(y.powf(x as f64))),
                |x: f64, y: f64| Ok(DinoRef::float(y.powf(x)))
            )
        },
        (value_type::STRING, value_type::STRING) => {
            string_op_string!(a, b, memory,
                |ai: i64, bi: i64| Ok(DinoRef::float((ai as f64).powf(bi as f64))),
                |af: f64, bf: f64| Ok(DinoRef::float(af.powf(bf)))
            )
        },
        (value_type::BOOL, value_type::BOOL) => {
            let a_val: f64 = if a.as_bool() { 1.0 } else { 0.0 };
            let b_val: f64 = if b.as_bool() { 1.0 } else { 0.0 };
            Ok(DinoRef::float(a_val.powf(b_val)))
        },
        _ => Err(RuntimeError::Typed(RuntimeErrorType::InvalidBinaryOperation {
            left: a.type_name().to_string(),
            op: "**".to_string(),
            right: b.type_name().to_string(),
        })),
    }
}
