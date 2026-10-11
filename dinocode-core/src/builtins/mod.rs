pub mod io;
pub mod types;
pub mod utils;
pub mod array;
pub mod string;
pub mod object;
pub mod function;
pub mod bound;
pub mod range;
pub mod time;
pub mod math;
pub mod r#type;
pub mod bigint;

pub fn init_all() {
    io::init_io();
    types::init_types();
    utils::init_utils();
    array::init_array();
    string::init_string();
    object::init_object();
    function::init_function();
    bound::init_bound_function();
    range::init_range();
    time::init_time();
    math::init_math();
    r#type::init_type();
}
