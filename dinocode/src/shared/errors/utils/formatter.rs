// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/shared/errors/utils/formatter.rs
//  Desc:       Pretty error formatting for lexer and parser errors.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use colored::Colorize;
use crate::{
    shared::{
        errors::{
            types::{ErrorInfo, ErrorCategory, LexErrorType},
            services as services,
        },
    },
    compiler::{
        parser::errors::{ParseError, ParseErrorType},
        lexer::errors::LexError,
    },
};
use dinocode_core::{
    errors::RuntimeError,
    utils::source_map::SourceMap,
};

pub fn pretty_format_error(error_info: ErrorInfo, line: usize, col: usize, source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let line_idx = line.saturating_sub(1);
    let start = line_idx.saturating_sub(2).max(0);
    let end = (line_idx + 3).min(lines.len());

    let error_label = match error_info.category {
        ErrorCategory::Lexer => "lex-error",
        ErrorCategory::Parser => "parse-error", 
        ErrorCategory::Runtime => "runtime-error",
        ErrorCategory::Compiler => "compiler-error",
    };

    let mut output = format!(
        "\n{}: {} {}\n\n", 
        error_label.red().bold(), 
        error_info.message,
        format!("(line {line}, col {col})").dimmed()
    );

    let max_width = end.to_string().len();

    for n in start..end {
        let line_num = n + 1;
        let marker = if line_num == line { "---> " } else { "     " };
        let line_content = lines[n];

        output += &format!("{marker}{:>width$}: {}\n", line_num, line_content, width = max_width);

        if line_num == line {
            let spaces = " ".repeat(marker.len() + max_width + 2 + col.saturating_sub(1));
            output += &format!("{}{}\n", spaces, "^".red());
        }
    }

    let label_padding = "     ";

    if let Some(suggestion) = error_info.suggestion {
        output += &format!("\n{}{}: {}\n", label_padding, "help".green().bold(), suggestion);
    }

    if let Some(details) = error_info.details {
        output += &format!("\n{}{}: {}\n", label_padding, "note".bright_blue().bold(), details);
    }

    output
}

fn format_frame_snippet(frame_line: usize, col: usize, source_lines: &[&str], indent: &str) -> String {
    let line_idx = frame_line.saturating_sub(1);
    let start = line_idx.saturating_sub(1);
    let end = (line_idx + 2).min(source_lines.len());
    let max_width = end.to_string().len();
    let mut out = String::new();

    for n in start..end {
        let ln = n + 1;
        let marker = if ln == frame_line {
            format!("{}---> ", indent)
        } else {
            format!("{}     ", indent)
        };
        out += &format!("{}{:>width$}: {}\n", marker, ln, source_lines[n], width = max_width);
        if ln == frame_line {
            let spaces = " ".repeat(indent.len() + 5 + max_width + 2 + col.saturating_sub(1));
            out += &format!("{}{}\n", spaces, "^".yellow());
        }
    }

    out
}

pub fn pretty_lex_error_from_info(error_info: ErrorInfo, line: usize, col: usize, source: &str) -> LexError {
    let formatted = pretty_format_error(error_info, line, col, source);
    LexError::Tokenize(formatted)
}

pub fn pretty_lex_error(error_type: LexErrorType, line: usize, col: usize, source: &str) -> LexError {
    let error_info = services::resolve_lex_error(error_type);
    pretty_lex_error_from_info(error_info, line, col, source)
}

pub fn pretty_lex_error_custom(message: String, suggestion: Option<String>, details: Option<String>, line: usize, col: usize, source: &str) -> LexError {
    let error_info = services::resolve_custom_lex_error(message, suggestion, details);
    pretty_lex_error_from_info(error_info, line, col, source)
}

pub fn pretty_parse_error_from_info(error_info: ErrorInfo, line: usize, col: usize, source: &str) -> ParseError {
    let formatted = pretty_format_error(error_info, line, col, source);
    ParseError::Parse(formatted)
}

pub fn pretty_parse_error(error_type: ParseErrorType, line: usize, col: usize, source: &str) -> ParseError {
    let error_info = services::resolve_parse_error(error_type);
    pretty_parse_error_from_info(error_info, line, col, source)
}

pub fn pretty_runtime_error_from_info(
    error: RuntimeError,
    call_stack_ips: &[usize],
    source_map: &SourceMap,
    source: &str,
) -> String {
    let (message, suggestion, details) = error.get_info();
    let error_info = ErrorInfo::custom(
        ErrorCategory::Runtime,
        message,
        suggestion,
        details,
    );

    let source_lines: Vec<&str> = source.lines().collect();

    let primary_ip = call_stack_ips.last().copied().unwrap_or(0);
    let (line, col) = source_map.get_location(primary_ip).unwrap_or((1, 1));

    let mut output = pretty_format_error(error_info, line, col, source);

    let caller_ips = if call_stack_ips.len() > 1 {
        &call_stack_ips[..call_stack_ips.len() - 1]
    } else {
        return output;
    };

    let frames: Vec<(usize, usize)> = caller_ips.iter().rev()
        .filter_map(|&ip| {
            let lookup_ip = ip.saturating_sub(1);
            source_map.get_location(lookup_ip)
        })
        .collect();

    if frames.is_empty() {
        return output;
    }

    let indent = "  ";
    let last_frame_idx = frames.len() - 1;

    for (i, &(frame_line, frame_col)) in frames.iter().enumerate() {
        let is_last = i == last_frame_idx;
        let label = if is_last {
            format!("called from line {}, col {}", frame_line, frame_col)
                .bright_black().bold().to_string()
        } else {
            format!("called from line {}, col {}", frame_line, frame_col)
                .dimmed().to_string()
        };
        output += &format!("\n{}  {}\n", indent, label);
        output += &format_frame_snippet(frame_line, frame_col, &source_lines, &format!("{}  ", indent));
    }

    output += "\n";
    output
}
