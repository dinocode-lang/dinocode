pub mod memory;
pub mod types;
pub mod native;
pub mod prototypes;
pub mod errors;
pub mod utils;
pub mod builtins;
pub mod formatter;

pub use types::DinoRef;
pub use memory::MemoryManager;
pub use errors::{RuntimeError, Result};
pub use formatter::*;

pub mod macros;

pub fn init() {
    #[cfg(feature = "logging")]
    {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();

        log::info!(" Initializing native functions...");
    }

    crate::builtins::init_all();
    crate::prototypes::init_all();
}
