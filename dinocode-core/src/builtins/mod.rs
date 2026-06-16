pub mod io;
pub mod types;
pub mod utils;

pub fn init_all() {
    io::init_io();
    types::init_types();
    utils::init_utils();
}
