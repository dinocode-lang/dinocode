mod error;
mod parse;
mod utils;
pub mod types;
mod strict;
mod lax;

pub use types::number::Number;
pub use error::{
    NumericParseError,
    error_i64,
    error_f64,
    error_hex,
    error_bin
};
pub use parse::{
    ParseNumeric,
    ParseNumericLax,
    parse,
    parse_lax
};
pub use utils::{
    is_valid_int,
    trim_whitespace,
    parse_i64_hex,
    parse_i64_bin,
    parse_i64_decimal,
    parse_i64_octal,
    parse_bigint_digits_bytes,
};
