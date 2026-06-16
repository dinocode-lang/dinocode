pub mod conversions;
pub mod registry;

pub use registry::{
    register_native_function,
    call_native_function,
    get_native_registry,
    NativeFunctionRegistry,
    register_native_class,
    free_info,
};

pub use registry::is_native_function;

pub use conversions::{FromDinoRef, ToDinoRef};

pub fn register_native_function_with_flags(name: &str, function: registry::NativeFnWrapper, flags: u8) -> u32 {
    registry::get_native_registry().register_with_flags(name, function, flags)
}
