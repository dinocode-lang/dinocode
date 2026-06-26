// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/types/dinoref.rs
//  Desc:       Range-Based NaN Boxing
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::{
    memory::MemoryManager,
    utils::TypeConverter,
    errors::{RuntimeError, RuntimeErrorType},
    types::Symbol,
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DinoRef(u64);

pub const MIN_TAGGED_VALUE: u64 = 0xFFF0000000000000;
pub const INT_BASE_RANGE: u64   = 0xFFFF000000000000;

const TAG_BIGINT: u64    = 0xFFF1000000000000;
const TAG_STRING: u64    = 0xFFF2000000000000;
const TAG_ARRAY: u64     = 0xFFF3000000000000;
const TAG_BOOL: u64      = 0xFFF4000000000000;
const TAG_NONE: u64      = 0xFFF5000000000000;
const TAG_OBJECT: u64    = 0xFFF6000000000000;
const TAG_FUNCTION: u64  = 0xFFF7000000000000;
const TAG_SYMBOL: u64    = 0xFFF8000000000000;

const FUNCTION_NATIVE_FLAG: u64 = 0x0000000000000001;
const OBJECT_CLASS_FLAG: u64    = 0x0000000000000001;

const EXPONENT_MASK: u64 = 0x7FF0000000000000;
const PAYLOAD_32_MASK: u64 = 0x00000000FFFFFFFF;
const PAYLOAD_48_MASK: u64 = 0x0000FFFFFFFFFFFF;
const TAG_CLEAR_MASK: u64  = 0xFFFF000000000000;
const SIGN_CLEAR_MASK: u64  = 0x7FFFFFFFFFFFFFFF;

// Special masks
const TAG_FUNCTION_MASK: u64 = TAG_FUNCTION;
const TAG_NATIVE_FN_MASK: u64 = TAG_FUNCTION | FUNCTION_NATIVE_FLAG;
const TAG_OBJECT_MASK: u64 = TAG_OBJECT;
const TAG_CLASS_MASK: u64 = TAG_OBJECT | OBJECT_CLASS_FLAG;

pub mod value_type {
    pub const FLOAT: u16 = 0x0000;  // (b < MIN_TAGGED_VALUE, no tagged)
    pub const BIGINT: u16   = 0xFFF1;  // TAG_BIGINT   >> 48
    pub const STRING: u16   = 0xFFF2;  // TAG_STRING   >> 48
    pub const ARRAY: u16    = 0xFFF3;  // TAG_ARRAY    >> 48
    pub const BOOL: u16     = 0xFFF4;  // TAG_BOOL     >> 48
    pub const NONE: u16     = 0xFFF5;  // TAG_NONE     >> 48
    pub const OBJECT: u16   = 0xFFF6;  // TAG_OBJECT   >> 48
    pub const FUNCTION: u16 = 0xFFF7;  // TAG_FUNCTION >> 48
    pub const SYMBOL: u16   = 0xFFF8;  // TAG_SYMBOL   >> 48
    pub const INT: u16 = 0xFFFF;       // INT_BASE_RANGE >> 48
}

impl DinoRef {

    pub const INT_MIN: i64 = -140737488355328;
    pub const INT_MAX: i64 = 140737488355327;
    pub const TRUE: Self = Self::bool(true);
    pub const FALSE: Self = Self::bool(false);
    pub const NONE: Self = Self::none();
    pub const ZERO: Self = Self::int(0);
    pub const ONE: Self = Self::int(1);
    pub const NAN: Self = Self(0x7FF8000000000000);
    pub const INFINITY: Self = Self(0x7FF0000000000000);
    pub const NEG_INFINITY: Self = Self(0xFFF0000000000000);

    // constructors
    #[inline(always)]
    pub const fn float(value: f64) -> Self {
        if value.is_nan() {
            Self::NAN
        } else {
            Self(value.to_bits())
        }
    }
    #[inline(always)] pub const fn int(value: i64) -> Self { Self(INT_BASE_RANGE | (value as u64 & PAYLOAD_48_MASK)) }
    #[inline(always)] pub const fn bigint(offset: u32) -> Self { Self(TAG_BIGINT | (offset as u64)) }
    #[inline(always)] pub const fn bool(value: bool) -> Self { Self(TAG_BOOL | if value { 1 } else { 0 }) }
    #[inline(always)] pub const fn none() -> Self { Self(TAG_NONE) }
    #[inline(always)] pub const fn symbol(id: u32) -> Self { Self(TAG_SYMBOL | (id as u64)) }
    #[inline(always)] pub const fn string(offset: u32) -> Self { Self(TAG_STRING | (offset as u64)) }
    #[inline(always)] pub const fn array(offset: u32) -> Self { Self(TAG_ARRAY | (offset as u64)) }
    #[inline(always)] pub const fn object(offset: u32) -> Self { Self(TAG_OBJECT | ((offset as u64) << 1)) }
    #[inline(always)] pub const fn class(offset: u32) -> Self { Self(TAG_OBJECT | ((offset as u64) << 1) | OBJECT_CLASS_FLAG) }
    #[inline(always)] pub const fn native_fn(offset: u32) -> Self { Self(TAG_FUNCTION | ((offset as u64) << 1) | FUNCTION_NATIVE_FLAG) }
    #[inline(always)] pub const fn function(offset: u32) -> Self { Self(TAG_FUNCTION | ((offset as u64) << 1)) }

    #[inline(always)] pub fn is_float(self) -> bool { self.0 <= MIN_TAGGED_VALUE }
    #[inline(always)] pub fn is_nan(self) -> bool { self.0 == Self::NAN.0 }
    #[inline(always)] pub fn is_inf(self) -> bool { (self.0 & SIGN_CLEAR_MASK) == EXPONENT_MASK }
    #[inline(always)] pub fn is_finite(self) -> bool { (self.0 & SIGN_CLEAR_MASK) < EXPONENT_MASK }
    #[inline(always)] pub fn is_number(self) -> bool { self.is_float() || self.is_int() || self.is_bigint() }
    #[inline(always)] pub fn is_int(self) -> bool { self.0 >= INT_BASE_RANGE }
    #[inline(always)] pub fn is_bigint(self) -> bool { (self.0 & TAG_CLEAR_MASK) == TAG_BIGINT }
    #[inline(always)] pub fn is_string(self) -> bool { (self.0 & TAG_CLEAR_MASK) == TAG_STRING }
    #[inline(always)] pub fn is_const(self) -> bool { let t = self.0 & TAG_CLEAR_MASK; t == TAG_STRING || t == TAG_BIGINT }
    #[inline(always)] pub fn is_array(self) -> bool { (self.0 & TAG_CLEAR_MASK) == TAG_ARRAY }
    #[inline(always)] pub fn is_bool(self) -> bool { (self.0 & TAG_CLEAR_MASK) == TAG_BOOL }
    #[inline(always)] pub fn is_none(self) -> bool { self.0 == TAG_NONE }
    #[inline(always)] pub fn is_symbol(self) -> bool { (self.0 & TAG_CLEAR_MASK) == TAG_SYMBOL }
    #[inline(always)] pub fn is_object(self) -> bool { (self.0 & TAG_CLEAR_MASK) == TAG_OBJECT }
    #[inline(always)] pub fn is_function(self) -> bool { (self.0 & TAG_CLEAR_MASK) == TAG_FUNCTION }
    #[inline(always)] pub fn is_native_fn(self) -> bool { (self.0 & TAG_NATIVE_FN_MASK) == TAG_NATIVE_FN_MASK }
    #[inline(always)] pub fn is_user_function(self) -> bool { (self.0 & TAG_NATIVE_FN_MASK) == TAG_FUNCTION_MASK }
    #[inline(always)] pub fn is_class(self) -> bool { (self.0 & TAG_CLASS_MASK) == TAG_CLASS_MASK }
    #[inline(always)] pub fn is_instance(self) -> bool { (self.0 & TAG_CLASS_MASK) == TAG_OBJECT_MASK }

    // Convert
    #[inline(always)] pub fn as_float(self) -> f64 { f64::from_bits(self.0) }
    #[inline(always)] pub fn as_int(self) -> i64 {((self.0 << 16) as i64) >> 16}
    #[inline(always)] pub fn as_bigint(self) -> u32 { (self.0 & PAYLOAD_32_MASK) as u32 }
    #[inline(always)] pub fn as_bool(self) -> bool { (self.0 & 1) != 0 }
    #[inline(always)] pub fn as_symbol(self) -> u32 { (self.0 & PAYLOAD_32_MASK) as u32 }

    #[inline(always)]
    pub fn as_finite_float(self) -> Result<f64, RuntimeError> {
        if !self.is_finite() {
            if self.is_nan() { return Err(RuntimeError::Typed(RuntimeErrorType::ValueIsNaN)); }
            return Err(RuntimeError::Typed(RuntimeErrorType::ValueIsInfinity));
        }
        Ok(self.as_float())
    }

    #[inline(always)]
    pub fn as_non_nan_float(self) -> Result<f64, RuntimeError> {
        if self.is_nan() { return Err(RuntimeError::Typed(RuntimeErrorType::ValueIsNaN)); }
        Ok(self.as_float())
    }

    #[inline(always)]
    pub fn try_as_float(self, memory: &mut MemoryManager) -> Result<f64, RuntimeError> {
        let vtype = self.decode_type();
        if vtype == value_type::FLOAT { return self.as_finite_float(); }
        TypeConverter::to_primitive_float(self, vtype, memory)
    }

    #[inline(always)]
    pub fn try_as_int(self, memory: &mut MemoryManager) -> Result<i64, RuntimeError> {
        let vtype = self.decode_type();
        if vtype == value_type::INT { return Ok(self.as_int()); }
        TypeConverter::to_primitive_int(self, vtype, memory)
    }

    #[inline(always)]
    pub fn try_as_string(self, memory: &mut MemoryManager) -> Result<String, RuntimeError> {
        let vtype = self.decode_type();
        if vtype == value_type::STRING { return Ok(memory.get_string(self.decode_index()).to_string()); }
        TypeConverter::to_primitive_string(self, vtype, memory)
    }

    #[inline(always)]
    pub fn try_as_bool(self, memory: &mut MemoryManager) -> Result<bool, RuntimeError> {
        let vtype = self.decode_type();
        if vtype == value_type::BOOL { return Ok(self.as_bool()); }
        TypeConverter::to_primitive_bool(self, vtype, memory)
    }

    #[inline(always)]
    pub fn to_display_string(self, memory: &MemoryManager) -> Result<String, RuntimeError> {
        TypeConverter::to_display_string(self, memory)
    }

    // Getters
    #[inline(always)]
    pub fn get_function_id(self) -> u32 {
        (self.0 >> 1) as u32
    }

    #[inline(always)]
    pub fn get_object_id(self) -> u32 {
        (self.0 >> 1) as u32
    }

    // Utilities
    #[inline(always)]
    pub const fn is_valid_int(v: i64) -> bool {
        v >= Self::INT_MIN && v <= Self::INT_MAX
    }

    #[inline(always)]
    pub fn is_valid_float(v: f64) -> bool {
        v.is_finite()
    }

    #[inline(always)]
    pub fn decode_type(self) -> u16 {
        let is_float = (self.0 <= MIN_TAGGED_VALUE) as u16;
        ((self.0 >> 48) as u16) & is_float.wrapping_sub(1)
    }
    
    #[inline(always)]
    pub fn decode_index(self) -> u32 { (self.0 & PAYLOAD_32_MASK) as u32 }

    #[inline(always)]
    pub fn type_name(self) -> &'static str {
        match self.decode_type() {
            value_type::INT => "int",
            value_type::BIGINT => "bigint",
            value_type::FLOAT => "float",
            value_type::BOOL => "bool",
            value_type::STRING => "string",
            value_type::ARRAY => "array",
            value_type::OBJECT => "object",
            value_type::NONE => "none",
            value_type::SYMBOL => "symbol",
            value_type::FUNCTION => "function",
            _ => "unknown",
        }
    }
    
    #[inline(always)] pub fn payload(self) -> u64 { self.0 & PAYLOAD_48_MASK }
    #[inline(always)] pub fn raw(self) -> u64 { self.0 }
    #[inline(always)] pub fn from_raw(raw: u64) -> Self { Self(raw) }

    #[inline(always)]
    pub fn with_payload(self, new_payload: u64) -> Self {
        let tag_bits = self.0 & TAG_CLEAR_MASK;
        Self(tag_bits | (new_payload & PAYLOAD_48_MASK))
    }
}

impl std::fmt::Display for DinoRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.decode_type() {
            value_type::INT => write!(f, "{}", self.as_int()),
            value_type::FLOAT => write!(f, "{}", self.as_float()),
            value_type::BOOL => write!(f, "{}", self.as_bool()),
            value_type::NONE => write!(f, "none"),
            value_type::SYMBOL => write!(f, "{}", Symbol::to_name(*self)),
            _ => write!(f, "[tag:{:x}:idx:{}]", self.0 >> 48, self.decode_index()),
        }
    }
}

impl From<i64> for DinoRef { fn from(v: i64) -> Self { Self::int(v) } }
impl From<i32> for DinoRef { fn from(v: i32) -> Self { Self::int(v as i64) } }
impl From<f64> for DinoRef { fn from(v: f64) -> Self { Self::float(v) } }
impl From<bool> for DinoRef { fn from(v: bool) -> Self { Self::bool(v) } }
impl From<()> for DinoRef { fn from(_: ()) -> Self { Self::none() } }
