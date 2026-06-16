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
            _ => None
        }
    }

    pub fn to_name(dinoref: DinoRef) -> Option<&'static str> {
        match dinoref {
            Self::NEW => Some("new"),
            Self::CALL => Some("call"),
            Self::ADD => Some("+"),
            Self::SUB => Some("-"),
            Self::MUL => Some("*"),
            Self::DIV => Some("/"),
            Self::MOD => Some("%"),
            Self::POW => Some("**"),
            Self::EQ => Some("=="),
            Self::NE => Some("!="),
            Self::LT => Some("<"),
            Self::LE => Some("<="),
            Self::GT => Some(">"),
            Self::GE => Some(">="),
            Self::IN => Some("in"),
            _ => None
        }
    }
}
