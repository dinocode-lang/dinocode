pub mod shared;
pub mod compiler;
pub mod interpreter;

pub use interpreter::VirtualMachine;
pub use dinocode_macros::dinof;
pub use dinocode_core::errors::RuntimeError;
pub use dinocode_core::{memory, types, native, errors, init};
