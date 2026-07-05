// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/formatter/renderer.rs
//  Desc:       Convert DinoError to formatted string
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use colored::Colorize;
use crate::formatter::{
    DinoError,
    ErrorGroup,
    ErrorType,
    FormatterColor,
};

pub struct ErrorRenderer<'a> {
    error: &'a DinoError<'a>,
    source: &'a str,
}

impl<'a> ErrorRenderer<'a> {
    pub fn new(error: &'a DinoError<'a>, source: &'a str) -> Self {
        Self { error, source }
    }

    pub fn render(&self) -> String {
        let lines: Vec<&str> = self.source.lines().collect();
        let line_idx = self.error.line.saturating_sub(1) as usize;
        let start = line_idx.saturating_sub(2);
        let end = (line_idx + 3).min(lines.len());

        let main_message = self.group_parts(ErrorGroup::Message);
        let max_width = end.to_string().len();

        let mut out = String::new();
        out.push('\n');
        let error_label = match self.error.error_type {
            ErrorType::Lexer => "lex-error",
            ErrorType::Parser => "parse-error",
            ErrorType::Runtime => "runtime-error",
        };
        out.push_str(&error_label.red().bold().to_string());
        out.push_str(": ");
        out.push_str(&main_message);
        out.push(' ');
        
        let mut loc = String::from("(line ");
        loc.push_str(&self.error.line.to_string());
        loc.push_str(", col ");
        loc.push_str(&self.error.column.to_string());
        loc.push(')');
        out.push_str(&loc.dimmed().to_string());
        out.push_str("\n\n");

        for n in start..end {
            let line_num = n + 1;
            let marker = if line_num == self.error.line as usize { "--> " } else { "    " };
            let line_content = lines[n];

            out.push_str(marker);
            let num_str = line_num.to_string();
            for _ in num_str.len()..max_width {
                out.push(' ');
            }
            out.push_str(&num_str);
            out.push_str(": ");
            out.push_str(line_content);
            out.push('\n');

            if line_num == self.error.line as usize {
                let spaces = marker.len() + max_width + 2 + self.error.column.saturating_sub(1) as usize;
                for _ in 0..spaces {
                    out.push(' ');
                }
                out.push_str(&"^".red().to_string());
                out.push('\n');
            }
        }

        let note_content = self.group_parts(ErrorGroup::Note);
        if !note_content.is_empty() {
            out.push_str("\n    ");
            out.push_str(&"help".green().bold().to_string());
            out.push_str(": ");
            out.push_str(&note_content);
            out.push('\n');
        }

        let info_content = self.group_parts(ErrorGroup::Info);
        if !info_content.is_empty() {
            out.push_str("\n    ");
            out.push_str(&"note".bright_blue().bold().to_string());
            out.push_str(": ");
            out.push_str(&info_content);
            out.push('\n');
        }

        if !self.error.stack_trace.is_empty() {
            let indent = "  ";
            let last_frame_idx = self.error.stack_trace.len().saturating_sub(1);
            
            for (i, frame) in self.error.stack_trace.iter().enumerate() {
                let is_last = i == last_frame_idx;
                let mut frame_label = String::from("called from line ");
                frame_label.push_str(&frame.line.to_string());
                frame_label.push_str(", col ");
                frame_label.push_str(&frame.column.to_string());
                let label = if is_last {
                    frame_label.bright_black().bold().to_string()
                } else {
                    frame_label.dimmed().to_string()
                };
                out.push('\n');
                out.push_str(indent);
                out.push_str("  ");
                out.push_str(&label);
                out.push('\n');
                let mut frame_indent = indent.to_string();
                frame_indent.push_str("  ");
                out.push_str(&self.format_frame_snippet(frame.line as usize, frame.column as usize, &lines, &frame_indent));
            }
        }

        out
    }

    fn format_frame_snippet(&self, frame_line: usize, col: usize, source_lines: &[&str], indent: &str) -> String {
        let line_idx = frame_line.saturating_sub(1);
        let start = line_idx.saturating_sub(1);
        let end = (line_idx + 2).min(source_lines.len());
        let max_width = end.to_string().len();
        let mut out = String::new();

        for n in start..end {
            let ln = n + 1;
            let is_target = ln == frame_line;
            out.push_str(indent);
            if is_target {
                out.push_str("---> ");
            } else {
                out.push_str("     ");
            }
            let ln_str = ln.to_string();
            for _ in ln_str.len()..max_width {
                out.push(' ');
            }
            out.push_str(&ln_str);
            out.push_str(": ");
            out.push_str(source_lines[n]);
            out.push('\n');
            if is_target {
                let spaces = " ".repeat(indent.len() + 5 + max_width + 2 + col.saturating_sub(1));
                out.push_str(&spaces);
                out.push_str(&"^".yellow().to_string());
                out.push('\n');
            }
        }

        out
    }

    fn group_parts(&self, group: ErrorGroup) -> String {
        let mut buf = String::new();
        for part in self.error.parts.iter().filter(|p| p.group == group) {
            if part.color != FormatterColor::Default {
                buf.push_str(part.color.ansi_code());
                buf.push_str(part.content.as_ref());
                buf.push_str(FormatterColor::Reset.ansi_code());
            } else {
                buf.push_str(part.content.as_ref());
            }
        }
        buf
    }
}
