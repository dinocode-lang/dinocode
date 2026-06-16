// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/analysis/cfg/types.rs
//  Desc:       Control Flow Graph types
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use serde::{Serialize, Deserialize};
use crate::bytecode::types::InstructionInfo;

/*
 * CFG Node
 * ------------------------------
 * Represents a basic block in the control flow graph with its instructions,
 * source text, node type (condition, process, terminal, etc.), and optional
 * called function reference for cross-function calls.
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct CFGNode {
    pub id: u32,
    pub start_ip: u32,
    pub end_ip: u32,
    pub instructions: Vec<InstructionInfo>,
    pub source_text: String,
    pub node_type: String,
    pub called_function_ref: Option<u64>,
}

/*
 * CFG Edge
 * ------------------------------
 * Represents a directed edge between two nodes with an optional label
 * (such as "true" or "false" for conditional branches).
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct CFGEdge {
    pub from: u32,
    pub to: u32,
    pub label: String,
}

/*
 * CFG Subgraph
 * ------------------------------
 * Represents a function's control flow as a subgraph with its own nodes,
 * edges, and optional Start/Fin terminal nodes.
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct CFGSubgraph {
    pub id: String,
    pub name: String,
    pub function_ref: Option<u64>,
    pub nodes: Vec<CFGNode>,
    pub edges: Vec<CFGEdge>,
}

/*
 * Control Flow Graph
 * ------------------------------
 * Complete CFG with main graph nodes/edges and function subgraphs.
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct ControlFlowGraph {
    pub nodes: Vec<CFGNode>,
    pub edges: Vec<CFGEdge>,
    pub subgraphs: Vec<CFGSubgraph>,
}
