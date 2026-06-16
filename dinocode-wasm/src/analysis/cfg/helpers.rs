// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/analysis/cfg/helpers.rs
//  Desc:       Helper functions for CFG construction
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use super::types::{CFGNode, CFGEdge};
use dinocode_core::types::opcode_defs::opcode::{
    JUMP,
    FOR_DROP,
    POP,
};

pub fn find_leader(leaders: &[u32], ip: u32) -> u32 {
    let mut last_leader = 0;
    for &leader in leaders {
        if leader <= ip {
            last_leader = leader;
        } else {
            break;
        }
    }
    last_leader
}

pub fn get_next_leader(leaders: &[u32], current_leader: u32) -> Option<u32> {
    for &leader in leaders {
        if leader > current_leader {
            return Some(leader);
        }
    }
    None
}

pub fn find_raw_successor(from_id: u32, raw_edges: &[CFGEdge]) -> Option<u32> {
    raw_edges.iter().find(|e| e.from == from_id).map(|e| e.to)
}

pub fn is_helper_node(node: &CFGNode) -> bool {
    if node.node_type == "input" || node.node_type == "output" || node.node_type == "terminal" || node.node_type == "condition" || node.node_type == "preparation" {
        return false;
    }
    node.instructions.iter().all(|instr| {
        matches!(instr.opcode, JUMP | FOR_DROP | POP)
    })
}
