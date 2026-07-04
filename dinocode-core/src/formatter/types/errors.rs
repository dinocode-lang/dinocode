// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/formatter/types/errors.rs
//  Desc:       DinoError structure
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::borrow::Cow;
use super::colors::FormatterColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    Lexer,
    Parser,
    Runtime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorGroup {
    Message,
    Note,
    Info,
}

pub struct MessageBlock<'a> {
    pub content: Cow<'a, str>,
    pub group: ErrorGroup,
    pub color: FormatterColor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackFrame {
    pub line: u32,
    pub column: u32,
}

pub struct DinoError<'a> {
    pub error_type: ErrorType,
    pub line: u32,
    pub column: u32,
    pub parts: Vec<MessageBlock<'a>>,
    pub stack_trace: Vec<StackFrame>,
}

impl<'a> DinoError<'a> {
    pub fn new(line: u32, column: u32) -> Self {
        Self {
            error_type: ErrorType::Runtime,
            line,
            column,
            parts: Vec::new(),
            stack_trace: Vec::new(),
        }
    }

    pub fn with_type(mut self, error_type: ErrorType) -> Self {
        self.error_type = error_type;
        self
    }

    pub fn add_stack_frame(mut self, line: u32, column: u32) -> Self {
        self.stack_trace.push(StackFrame { line, column });
        self
    }

    pub fn add_message(mut self, content: &'a str, color: FormatterColor) -> Self {
        self.parts.push(MessageBlock {
            content: Cow::Borrowed(content),
            group: ErrorGroup::Message,
            color,
        });
        self
    }

    pub fn add_message_owned(mut self, content: String, color: FormatterColor) -> Self {
        self.parts.push(MessageBlock {
            content: Cow::Owned(content),
            group: ErrorGroup::Message,
            color,
        });
        self
    }

    pub fn add_note(mut self, content: &'a str, color: FormatterColor) -> Self {
        self.parts.push(MessageBlock {
            content: Cow::Borrowed(content),
            group: ErrorGroup::Note,
            color,
        });
        self
    }

    pub fn add_note_owned(mut self, content: String, color: FormatterColor) -> Self {
        self.parts.push(MessageBlock {
            content: Cow::Owned(content),
            group: ErrorGroup::Note,
            color,
        });
        self
    }

    pub fn add_info(mut self, content: &'a str, color: FormatterColor) -> Self {
        self.parts.push(MessageBlock {
            content: Cow::Borrowed(content),
            group: ErrorGroup::Info,
            color,
        });
        self
    }

    pub fn add_info_owned(mut self, content: String, color: FormatterColor) -> Self {
        self.parts.push(MessageBlock {
            content: Cow::Owned(content),
            group: ErrorGroup::Info,
            color,
        });
        self
    }

    pub fn render(&self, source: &str) -> String {
        crate::renderer::ErrorRenderer::new(self, source).render()
    }
}
