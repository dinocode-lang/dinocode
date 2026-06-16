// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/shared/utils/operators.rs
//  Desc:       Defines helper functions for 
//              all recognized operators.
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use crate::shared::types::{Operator, TokenType};

pub struct Operators;

impl Operators {
    pub fn from_str(s: &str) -> Result<Operator, String> {
        match s {
            "+"   => Ok(Operator::Add),
            "-"   => Ok(Operator::Sub),
            "*"   => Ok(Operator::Mul),
            "/"   => Ok(Operator::Div),
            "//"  => Ok(Operator::FloorDiv),
            "%"   => Ok(Operator::Mod),
            "**"  => Ok(Operator::Pow),

            "++"  => Ok(Operator::Inc),
            "--"  => Ok(Operator::Dec),

            "="   => Ok(Operator::Assign),
            "+="  => Ok(Operator::AddAssign),
            "-="  => Ok(Operator::SubAssign),
            "*="  => Ok(Operator::MulAssign),
            "/="  => Ok(Operator::DivAssign),
            "//=" => Ok(Operator::FloorDivAssign),
            ".="  => Ok(Operator::ConcatAssign),
            "<-"  => Ok(Operator::Arrow),

            "=="  => Ok(Operator::Eq),
            "!="  => Ok(Operator::Ne),
            "=~"  => Ok(Operator::Match),
            ">"   => Ok(Operator::Gt),
            "<"   => Ok(Operator::Lt),
            ">="  => Ok(Operator::Ge),
            "<="  => Ok(Operator::Le),

            "&&" | "and" => Ok(Operator::And),
            "||" | "or" => Ok(Operator::Or),
            "!" | "not" => Ok(Operator::Not),

            "&"   => Ok(Operator::BitAnd),
            "|"   => Ok(Operator::BitOr),
            "^"   => Ok(Operator::BitXor),
            "<<"  => Ok(Operator::Shl),
            ">>"  => Ok(Operator::Shr),

            "."   => Ok(Operator::Dot),
            ":"   => Ok(Operator::Colon),
            "?"   => Ok(Operator::Question),
            ".\\"   => Ok(Operator::Backdot),

            "::" => Ok(Operator::BiColon),

            "as"  => Ok(Operator::As),

            ".." => Ok(Operator::Range),
            "..=" => Ok(Operator::RangeInclusive),

            _ => Err(format!("Unknown operator: {}", s)),
        }
    }

    pub const fn can_breaker(op: Operator) -> bool {
        matches!(op, Operator::Inc | Operator::Dec)
    }

    pub const fn is_assign(op: Operator) -> bool {
        matches!(
            op,
            Operator::Assign | Operator::AddAssign | Operator::SubAssign |
            Operator::MulAssign | Operator::DivAssign | Operator::FloorDivAssign |
            Operator::ConcatAssign | Operator::Arrow | Operator::Inc | Operator::Dec
        )
    }

    pub const fn is_assign_token(tt: TokenType) -> bool {
        matches!(tt, TokenType::Op(Operator::Assign) | TokenType::Op(Operator::AddAssign) |
        TokenType::Op(Operator::SubAssign) | TokenType::Op(Operator::MulAssign) | TokenType::Op(Operator::DivAssign) |
        TokenType::Op(Operator::FloorDivAssign) | TokenType::Op(Operator::ConcatAssign) | TokenType::Op(Operator::Arrow) |
        TokenType::Op(Operator::Inc) | TokenType::Op(Operator::Dec))
    }

    pub const fn precedence(op: Operator, unary: bool) -> i32 {
        match op {
            Operator::MatchIs => 1,
            
            Operator::Assign | Operator::AddAssign | Operator::SubAssign |
            Operator::MulAssign | Operator::DivAssign | Operator::FloorDivAssign |
            Operator::ConcatAssign | Operator::Arrow => 2,

            Operator::Or  => 4,
            Operator::And => 5,
            Operator::Question | Operator::Colon => 5,

            Operator::Eq | Operator::Ne | Operator::Match => 6,
            Operator::Gt | Operator::Lt | Operator::Ge | Operator::Le => 7,

            Operator::Dot => 8,

            Operator::Range | Operator::RangeInclusive => 8,

            Operator::BitAnd | Operator::BitOr | Operator::BitXor => 9,
            Operator::Shl | Operator::Shr => 10,
            
            Operator::Add | Operator::Sub if !unary => 11,
            Operator::Mul | Operator::Div | Operator::FloorDiv | Operator::Mod => 12,
            
            Operator::Not => 13,
            Operator::Pow => 14,
            Operator::Inc | Operator::Dec => 15,

            Operator::As => 16,
            
            _ if unary => 13,
            _ => 10,
        }
    }
}
