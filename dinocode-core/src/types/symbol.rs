use crate::types::dinoref::DinoRef;
use dinocode_macros::symbol_id;

pub struct Symbol;

impl Symbol {
    #[symbol_id]
    pub const NEW: DinoRef = ();
    
    #[symbol_id]
    pub const CALL: DinoRef = ();
    
    #[symbol_id]
    pub const ADD: DinoRef = ();
    
    #[symbol_id]
    pub const SUB: DinoRef = ();

    #[symbol_id]
    pub const MUL: DinoRef = ();
    
    #[symbol_id]
    pub const DIV: DinoRef = ();
    
    #[symbol_id]
    pub const MOD: DinoRef = ();
    
    #[symbol_id]
    pub const POW: DinoRef = ();
    
    #[symbol_id]
    pub const EQ: DinoRef = ();
    
    #[symbol_id]
    pub const NE: DinoRef = ();
    
    #[symbol_id]
    pub const LT: DinoRef = ();
    
    #[symbol_id]
    pub const LE: DinoRef = ();
    
    #[symbol_id]
    pub const GT: DinoRef = ();
    
    #[symbol_id]
    pub const GE: DinoRef = ();
    
    #[symbol_id]
    pub const IN: DinoRef = ();
    
    #[symbol_id]
    pub const FN: DinoRef = ();
    
    #[symbol_id]
    pub const ARGS: DinoRef = ();
    
    pub fn from_name(name: &str) -> Option<DinoRef> {
        match name {
            "new" => Some(Self::NEW),
            "call" => Some(Self::CALL),
            "+" => Some(Self::ADD),
            "-" => Some(Self::SUB),
            "*" => Some(Self::MUL),
            "/" => Some(Self::DIV),
            "%" => Some(Self::MOD),
            "**" => Some(Self::POW),
            "==" => Some(Self::EQ),
            "!=" => Some(Self::NE),
            "<" => Some(Self::LT),
            "<=" => Some(Self::LE),
            ">" => Some(Self::GT),
            ">=" => Some(Self::GE),
            "in" => Some(Self::IN),
            "fn" => Some(Self::FN),
            "args" => Some(Self::ARGS),
            _ => None
        }
    }

    pub fn to_name(dinoref: DinoRef) -> String {
        match dinoref {
            Self::NEW => "new",
            Self::CALL => "call",
            Self::ADD => "+",
            Self::SUB => "-",
            Self::MUL => "*",
            Self::DIV => "/",
            Self::MOD => "%",
            Self::POW => "**",
            Self::EQ => "==",
            Self::NE => "!=",
            Self::LT => "<",
            Self::LE => "<=",
            Self::GT => ">",
            Self::GE => ">=",
            Self::IN => "in",
            Self::FN => "fn",
            Self::ARGS => "args",
            _ => "unknown"
        }.to_string()
    }
}
