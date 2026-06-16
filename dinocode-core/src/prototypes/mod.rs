pub mod array;
pub mod object;
pub mod time;
pub mod range;
pub mod string;
pub mod r#type;
pub mod math;

crate::define_prototypes! {
    array => crate::prototypes::array::Array,
    object => crate::prototypes::object::Object,
    time => crate::prototypes::time::Time,
    range => crate::prototypes::range::Range,
    string => crate::prototypes::string::String,
    r#type => crate::prototypes::r#type::Type,
}

pub fn init_all() {
    array::init_array();
    object::init_object();
    time::init_time();
    range::init_range();
    string::init_string();
    r#type::init_type();
    math::init_math();
}
