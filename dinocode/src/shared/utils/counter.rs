// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/shared/utils/counter.rs
//  Desc:       Utility struct used to track numerical counts 
//              at different nested levels 
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Default)]
pub struct Counter {
    levels: Vec<usize>,
}

impl Counter {
    pub fn new() -> Self {
        Self { levels: vec![0] }
    }

    pub fn add(&mut self) {
        if let Some(last) = self.levels.last_mut() {
            *last += 1;
        }
    }

    pub fn get(&self) -> usize {
        *self.levels.last().unwrap_or(&0)
    }

    pub fn set(&mut self, value: usize) {
        if let Some(last) = self.levels.last_mut() {
            *last = value;
        }
    }

    pub fn depth(&self) -> usize {
        self.levels.len()
    }

    pub fn enter(&mut self) {
        self.levels.push(0);
    }

    pub fn leave(&mut self) {
        if self.levels.len() > 1 {
            self.levels.pop();
        }
    }

    pub fn is_even(&self) -> bool {
        self.get() % 2 == 0
    }

    pub fn is_odd(&self) -> bool {
        self.get() % 2 != 0
    }

    pub fn is_zero(&self) -> bool {
        self.get() == 0
    }

    pub fn push(&mut self, value: usize) {
        self.levels.push(value);
    }

    pub fn pop(&mut self) -> usize {
        self.levels.pop().unwrap_or(0)
    }
}
