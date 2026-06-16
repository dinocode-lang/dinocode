// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/shared/types/operators.rs
//  Desc:       Operator definitions.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Add, Sub, Mul, Div, FloorDiv, Mod, Pow,
    Inc, Dec,
    Assign, AddAssign, SubAssign, MulAssign, DivAssign, FloorDivAssign, ConcatAssign, Arrow,
    Eq, Ne, Gt, Lt, Ge, Le, Match, MatchIs,
    And, Or, Not,
    BitAnd, BitOr, BitXor, BitNot, Shl, Shr,
    Dot, Colon, Question, BiColon, Backdot,
    SetMember, SetIndex,
    As,
    Range, RangeInclusive,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::FloorDiv => "//",
            Operator::Mod => "%",
            Operator::Pow => "**",
            Operator::Inc => "++",
            Operator::Dec => "--",
            Operator::Assign => "=",
            Operator::AddAssign => "+=",
            Operator::SubAssign => "-=",
            Operator::MulAssign => "*=",
            Operator::DivAssign => "/=",
            Operator::FloorDivAssign => "//=",
            Operator::ConcatAssign => ".=",
            Operator::Arrow => "<-",
            Operator::Eq => "==",
            Operator::Ne => "!=",
            Operator::Match => "=~",
            Operator::MatchIs => "is",
            Operator::Gt => ">",
            Operator::Lt => "<",
            Operator::Ge => ">=",
            Operator::Le => "<=",
            Operator::And => "&&",
            Operator::Or => "||",
            Operator::Not => "!",
            Operator::BitAnd => "&",
            Operator::BitOr => "|",
            Operator::BitXor => "^",
            Operator::BitNot => "~",
            Operator::Shl => "<<",
            Operator::Shr => ">>",
            Operator::Dot => ".",
            Operator::Colon => ":",
            Operator::BiColon => "::",
            Operator::Question => "?",
            Operator::Backdot => "@",
            Operator::SetMember => "[]=",
            Operator::SetIndex => "[]=",
            Operator::As => "as",
            Operator::Range => "..",
            Operator::RangeInclusive => "..=",
        };
        write!(f, "{}", s)
    }
}
