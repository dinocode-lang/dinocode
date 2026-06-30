// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/utils/parsers/numeric/error.rs
//  Desc:       Numeric parse errors
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumericParseError {
    pub message: String,
    pub help: Option<String>,
    pub info: Option<String>,
}

impl NumericParseError {
    pub fn new(message: String, help: Option<String>, info: Option<String>) -> Self {
        Self { message, help, info }
    }
}

#[cold]
pub fn error_i64(bytes: &[u8]) -> NumericParseError {
    if bytes.is_empty() {
        return NumericParseError::new(
            "cannot convert empty string to number".to_string(),
            None,
            None,
        );
    }

    let limit = bytes.len().min(40);
    let mut has_digits = false;
    let mut iter = bytes[..limit].iter().peekable();

    if let Some(&&b) = iter.peek() {
        if b == b'-' || b == b'+' {
            iter.next();
        }
    }

    for &b in iter {
        if !b.is_ascii_digit() {
            return NumericParseError::new(
                "invalid integer format".to_string(),
                None,
                None,
            );
        }
        has_digits = true;
    }

    if !has_digits {
        return NumericParseError::new(
            "invalid integer format".to_string(),
            None,
            None,
        );
    }

    NumericParseError::new(
        "integer number too large".to_string(),
        Some("Try converting it to 'bigint'".to_string()),
        Some("Valid integers range from -140737488355328 to 140737488355327".to_string()),
    )
}

#[cold]
pub fn error_f64(bytes: &[u8]) -> NumericParseError {
    if bytes.is_empty() {
        return NumericParseError::new(
            "cannot convert empty string to number".to_string(),
            None,
            None,
        );
    }

    let limit = bytes.len().min(40);
    let mut dot_count = 0;
    let mut iter = bytes[..limit].iter().peekable();

    if let Some(&&b) = iter.peek() {
        if b == b'-' || b == b'+' {
            iter.next();
        }
    }

    for &b in iter {
        if b == b'.' {
            dot_count += 1;
            if dot_count > 1 {
                return NumericParseError::new(
                    "invalid float format".to_string(),
                    None,
                    None,
                );
            }
        } else if b == b'e' || b == b'E' {}
        else if !b.is_ascii_digit() && b != b'-' && b != b'+' {
            return NumericParseError::new(
                "invalid float format".to_string(),
                None,
                None,
            );
        }
    }

    NumericParseError::new(
        "invalid float format".to_string(),
        None,
        None,
    )
}

#[cold]
pub fn error_hex(bytes: &[u8]) -> NumericParseError {
    if bytes.is_empty() {
        return NumericParseError::new(
            "cannot convert empty string to number".to_string(),
            None,
            None,
        );
    }

    let limit = bytes.len().min(40);
    for &b in bytes[..limit].iter() {
        if !b.is_ascii_hexdigit() {
            return NumericParseError::new(
                "invalid hexadecimal format".to_string(),
                None,
                None,
            );
        }
    }

    NumericParseError::new(
        "hexadecimal number too large".to_string(),
        Some("Try converting it to 'bigint'".to_string()),
        Some("Valid integers range from -140737488355328 to 140737488355327".to_string()),
    )
}

#[cold]
pub fn error_bin(bytes: &[u8]) -> NumericParseError {
    if bytes.is_empty() {
        return NumericParseError::new(
            "cannot convert empty string to number".to_string(),
            None,
            None,
        );
    }

    let limit = bytes.len().min(40);
    for &b in bytes[..limit].iter() {
        if b != b'0' && b != b'1' {
            return NumericParseError::new(
                "invalid binary format".to_string(),
                None,
                None,
            );
        }
    }

    NumericParseError::new(
        "binary number too large".to_string(),
        Some("Try converting it to 'bigint'".to_string()),
        Some("Valid integers range from -140737488355328 to 140737488355327".to_string()),
    )
}

#[cold]
pub fn error_octal(bytes: &[u8]) -> NumericParseError {
    if bytes.is_empty() {
        return NumericParseError::new(
            "cannot convert empty string to number".to_string(),
            None,
            None,
        );
    }

    let limit = bytes.len().min(40);
    for &b in bytes[..limit].iter() {
        if b < b'0' || b > b'7' {
            return NumericParseError::new(
                "invalid octal format".to_string(),
                None,
                None,
            );
        }
    }

    NumericParseError::new(
        "octal number too large".to_string(),
        Some("Try converting it to 'bigint'".to_string()),
        Some("Valid integers range from -140737488355328 to 140737488355327".to_string()),
    )
}
