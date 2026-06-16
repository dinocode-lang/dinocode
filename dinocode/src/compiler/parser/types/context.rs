// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/parser/utils/context.rs
//  Desc:       Parser context for managing state during parsing.
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use dinocode_core::{
    utils::value_pool::ValuePool,
    memory::MemoryManager,
    utils::source_map::SourceMap,
    types::DinoRef,
};
use crate::compiler::parser::types::frames::{
    CondFrame,
    FuncFrame,
    ArrayFrame,
    ObjectFrame,
    FunctionDefFrame,
    ClassFrame,
    GroupFrame,
    LogicFrame,
};
use crate::shared::{
    types::Token,
    utils::TypeResolver,
};

pub struct ParserContext<'a> {
    pub instructions: Vec<u32>,
    pub source_map: SourceMap,
    pub memory_manager: MemoryManager,
    pub value_pool: ValuePool,
    pub op_stack: Vec<Token>,
    pub cond_frames: Vec<CondFrame>,
    pub logic_frames: Vec<LogicFrame>,
    pub func_frames: Vec<FuncFrame>,
    pub function_def_frames: Vec<FunctionDefFrame>,
    pub array_frames: Vec<ArrayFrame>,
    pub object_frames: Vec<ObjectFrame>,
    pub class_frames: Vec<ClassFrame>,
    pub group_frames: Vec<GroupFrame>,
    pub type_resolver: TypeResolver,
    pub source: &'a str,
    pub allow_main: bool,
    pub main_function: Option<DinoRef>,
}

impl<'a> ParserContext<'a> {
    pub fn new(tokens_len: usize, source: &'a str) -> Self {
        let mut ctx = Self {
            instructions: Vec::with_capacity(tokens_len),
            source_map: SourceMap::new(),
            memory_manager: MemoryManager::new(),
            value_pool: ValuePool::with_capacity(tokens_len / 4),
            op_stack: Vec::with_capacity(32),
            cond_frames: Vec::with_capacity(8),
            logic_frames: Vec::with_capacity(8),
            func_frames: Vec::with_capacity(16),
            function_def_frames: Vec::with_capacity(16),
            array_frames: Vec::with_capacity(16),
            object_frames: Vec::with_capacity(16),
            class_frames: Vec::with_capacity(16),
            group_frames: Vec::with_capacity(16),
            type_resolver: TypeResolver::new(),
            source,
            allow_main: true,
            main_function: None,
        };

        // Pre-allocate common constants
        ctx.value_pool.get_or_create_string("", &mut ctx.memory_manager);
        ctx.value_pool.get_or_create_int(1, &mut ctx.memory_manager);

        ctx
    }

    pub fn call_with_depth(&mut self, depth: u32) -> Option<FuncFrame> {
        self.func_frames.last().filter(|f| f.depth == depth)?;
        self.func_frames.pop()
    }

    pub fn object_with_depth(&mut self, depth: u32) -> Option<ObjectFrame> {
        self.object_frames.last().filter(|f| f.depth == depth)?;
        self.object_frames.pop()
    }

    pub fn array_with_depth(&mut self, depth: u32) -> Option<ArrayFrame> {
        self.array_frames.last().filter(|f| f.depth == depth)?;
        self.array_frames.pop()
    }

    pub fn group_with_depth(&mut self, depth: u32) -> Option<GroupFrame> {
        self.group_frames.last().filter(|f| f.depth == depth)?;
        self.group_frames.pop()
    }

    pub fn is_call_depth(&self, depth: u32) -> bool {
        self.func_frames.last().map_or(false, |f| f.depth == depth)
    }

    pub fn is_object_depth(&self, depth: u32) -> bool {
        self.object_frames.last().map_or(false, |f| f.depth == depth)
    }

    pub fn is_array_depth(&self, depth: u32) -> bool {
        self.array_frames.last().map_or(false, |f| f.depth == depth)
    }

    pub fn is_group_depth(&self, depth: u32) -> bool {
        self.group_frames.last().map_or(false, |f| f.depth == depth)
    }

    pub fn is_logic_depth(&self, depth: u32) -> bool {
        self.logic_frames.last().map_or(false, |f| f.depth == depth)
    }

    pub fn emit(&mut self, inst: u32, token: Option<&Token>) {
        if let Some(tok) = token {
            let line = tok.line.unwrap_or(0) as usize;
            let col = tok.column.unwrap_or(0) as usize;
            
            if line > 0 && col > 0 {
                self.source_map.insert_mapping(self.instructions.len(), line, col);
            }
        }
        self.instructions.push(inst);
    }
}
