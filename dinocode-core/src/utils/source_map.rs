// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/source_map.rs
//  Desc:       Manages metadata for mapping bytecode to source code locations.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq)]
pub struct SourceEntry {
    pub delta_ip: u16,
    pub delta_line: i16,
    pub delta_col: i16,
}

#[derive(Debug, Clone)]
pub struct ChunkAnchor {
    pub entry_start: u32,
    pub abs_ip: u32,
    pub abs_line: u32,
    pub abs_col: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SourceMap {
    pub entries: Vec<SourceEntry>,
    pub chunks: Vec<ChunkAnchor>,
    last_ip: u32,
    last_line: u32,
    last_col: u32,
}

impl SourceMap {
    const CHUNK_SIZE: u32 = 256;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_mapping(&mut self, instruction_index: usize, line: usize, column: usize) {
        let ip = instruction_index as u32;
        let line = line as u32;
        let col = column as u32;

        if self.last_col > 0 && self.last_line == line && self.last_col == col {
            return;
        }

        let chunk_id = (ip / Self::CHUNK_SIZE) as usize;
        while self.chunks.len() <= chunk_id {
            self.chunks.push(ChunkAnchor {
                entry_start: self.entries.len() as u32,
                abs_ip: self.last_ip,
                abs_line: self.last_line,
                abs_col: self.last_col,
            });
        }

        let delta_ip = ip.saturating_sub(self.last_ip).min(u16::MAX as u32) as u16;
        let delta_line = (line as i32 - self.last_line as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        let delta_col = (col as i32 - self.last_col as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16;

        self.entries.push(SourceEntry { delta_ip, delta_line, delta_col });

        self.last_ip = ip;
        self.last_line = line;
        self.last_col = col;
    }

    pub fn get_location(&self, instruction_index: usize) -> Option<(usize, usize)> {
        if self.entries.is_empty() {
            return None;
        }

        let ip = instruction_index as u32;
        let chunk_id = ((ip / Self::CHUNK_SIZE) as usize).min(self.chunks.len() - 1);
        let anchor = &self.chunks[chunk_id];

        let mut cur_ip = anchor.abs_ip;
        let mut cur_line = anchor.abs_line;
        let mut cur_col = anchor.abs_col;

        for entry in &self.entries[anchor.entry_start as usize..] {
            let next_ip = cur_ip + entry.delta_ip as u32;
            if next_ip > ip {
                break;
            }
            cur_ip = next_ip;
            cur_line = (cur_line as i32 + entry.delta_line as i32) as u32;
            cur_col = (cur_col as i32 + entry.delta_col as i32) as u32;
        }

        if cur_col > 0 {
            Some((cur_line as usize, cur_col as usize))
        } else {
            None
        }
    }
}
