use crate::utils::StringInterner;

#[derive(Debug)]
pub struct Arena {
    pub data: Vec<u8>,
    pub globals_start: usize,
    pub string_interner: StringInterner,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(1024 * 1024),
            globals_start: 0,
            string_interner: StringInterner::new(),
        }
    }

    pub fn set_globals(&mut self) {
        self.globals_start = self.data.len();
        self.string_interner.set_globals();
    }

    pub fn get_bytes(&self, offset: usize) -> &[u8] {
        let len = self.get_len(offset);
        &self.data[offset + 24..offset + 24 + len]
    }
}
