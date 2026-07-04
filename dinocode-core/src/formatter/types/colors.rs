// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/formatter/types/colors.rs
//  Desc:       Colors used by DinoError
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatterColor {
    Default,
    WhiteBold,
    Yellow,
    Green,
    RedBold,
    BrightBlueBold,
    Dimmed,
    Reset,
}

impl FormatterColor {
    pub fn ansi_code(self) -> &'static str {
        match self {
            FormatterColor::Default => "\x1b[0m",
            FormatterColor::WhiteBold => "\x1b[1;37m",
            FormatterColor::Yellow => "\x1b[33m",
            FormatterColor::Green => "\x1b[32m",
            FormatterColor::RedBold => "\x1b[1;31m",
            FormatterColor::BrightBlueBold => "\x1b[1;94m",
            FormatterColor::Dimmed => "\x1b[2m",
            FormatterColor::Reset => "\x1b[0m",
        }
    }
}