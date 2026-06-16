pub mod string;
pub mod buffer;
pub mod headers;
pub mod char_utils;

pub use string::{handle_escape, handle_interpolation};
pub use headers::{get_header_len};
pub use buffer::{
    push_number, 
    push_hex, 
    push_bit, 
    push_bigint,
    push_scientific, 
    push_ident, 
    push_dollar,
    push_operator,
    push_buffer,
};
pub use char_utils::{
    is_ident_start,
    is_ident, 
    is_interpolation_ident_start,
    is_interpolation_ident,
    is_digit, 
    is_hex_digit, 
    is_binary_digit, 
    is_sci_digit, 
    is_op_start,
    is_bigint_posfix,
};
