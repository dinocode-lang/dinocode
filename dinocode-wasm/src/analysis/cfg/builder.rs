// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/analysis/cfg/builder.rs
//  Desc:       Control Flow Graph builder
//  
//  Author:     Ismael Quiroz
//  Copyright:  (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

use std::collections::{
    BTreeSet,
    BTreeMap,
};
use dinocode_core::types::opcode_defs::opcode::{
    JUMP,
    JUMP_IF,
    JUMP_IF_NOT,
    RETURN,
    RETURN_REF,
    RETURN_SELF,
    HALT,
    POP,
    INPUT,
    FOR_INIT,
    CALL,
    FOR_ITER,
    FOR_ITER_ARRAY,
    FOR_ITER_RANGE,
    FOR_ITER_STRING,
    FOR_DROP,
    LOAD_CONST,
    GET_GLOBAL,
    SET_GLOBAL,
    GET_LOCAL,
    SET_LOCAL,
};
use crate::bytecode::types::{
    BytecodeInfo,
    InstructionInfo,
};
use crate::analysis::helpers::{
    is_print_call,
    get_source_range,
    get_print_ref,
};
use super::types::{
    CFGNode,
    CFGEdge,
    CFGSubgraph,
    ControlFlowGraph,
};
use super::helpers::{
    find_leader,
    get_next_leader,
    find_raw_successor,
    is_helper_node,
};

pub fn build_cfg(info: &BytecodeInfo, source: &str) -> ControlFlowGraph {
    /*
     * Function identification
     * ------------------------------
     * User functions have start_ip > 0. The JUMP that skips the body is at start_ip - 1,
     * and the LOAD_CONST immediately after end_ip carries the function DinoRef.
     */
    struct FunctionEntry {
        index: usize,
        start_ip: u32,
        end_ip: u32,
        jump_ip: u32,
        function_ref: Option<u64>,
    }

    let dyn_functions: Vec<FunctionEntry> = info.functions.iter().enumerate()
        .filter(|(_, f)| f.start_ip > 0)
        .map(|(i, f)| {
            let jump_ip = f.start_ip - 1;
            let function_ref = info.instructions.get((f.end_ip + 1) as usize)
                .filter(|instr| instr.opcode == LOAD_CONST)
                .and_then(|instr| info.constants.get(instr.operand as usize))
                .filter(|c| c.const_type == "function" || c.const_type == "class")
                .and_then(|c| c.raw);
            FunctionEntry { index: i, start_ip: f.start_ip, end_ip: f.end_ip, jump_ip, function_ref }
        })
        .collect();

    /*
     * Variable mapping
     * ------------------------------
     * Map global/local variables to their function references
     */
    let mut global_functions = BTreeMap::new();
    let mut local_functions = BTreeMap::new();
    for (i, instr) in info.instructions.iter().enumerate() {
        if instr.opcode == SET_GLOBAL || instr.opcode == SET_LOCAL {
            if i > 0 {
                let prev = &info.instructions[i - 1];
                if prev.opcode == LOAD_CONST {
                    if let Some(c) = info.constants.get(prev.operand as usize) {
                        if c.const_type == "function" || c.const_type == "class" {
                            if let Some(r) = c.raw {
                                if instr.opcode == SET_GLOBAL {
                                    global_functions.insert(instr.operand, r);
                                } else {
                                    local_functions.insert(instr.operand, r);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut potential_leaders = BTreeSet::new();

    if !info.instructions.is_empty() {
        potential_leaders.insert(0);
    }

    for func in &dyn_functions {
        potential_leaders.insert(func.start_ip);
    }
    
    for instr in &info.instructions {
        match instr.opcode {
            JUMP | JUMP_IF | JUMP_IF_NOT => {
                potential_leaders.insert(instr.operand);
                if instr.ip + 1 < info.instruction_count {
                    potential_leaders.insert(instr.ip + 1);
                }
            }
            RETURN | RETURN_REF | RETURN_SELF | HALT | POP => {
                if instr.ip + 1 < info.instruction_count {
                    potential_leaders.insert(instr.ip + 1);
                }
            }
            _ => {}
        }
    }
    
    let mut leaders = BTreeSet::new();
    if !info.instructions.is_empty() {
        leaders.insert(0);
    }
    for &ip in &potential_leaders {
        leaders.insert(ip);
    }
    
    let leaders_vec: Vec<u32> = leaders.into_iter().collect();
    let mut nodes_map: BTreeMap<u32, CFGNode> = BTreeMap::new();
    
    for i in 0..leaders_vec.len() {
        let start_ip = leaders_vec[i];
        let end_ip = if i + 1 < leaders_vec.len() {
            leaders_vec[i + 1] - 1
        } else {
            if info.instruction_count > 0 {
                info.instruction_count - 1
            } else {
                0
            }
        };
        
        let mut min_line = u32::MAX;
        let mut max_line = 0;
        for ip in start_ip..=end_ip {
            if let Some(instr) = info.instructions.get(ip as usize) {
                if instr.source_line > 0 {
                    if instr.opcode == FOR_DROP || instr.opcode == JUMP || (instr.opcode == POP && start_ip != end_ip) {
                        continue;
                    }
                    if instr.source_line < min_line { min_line = instr.source_line; }
                    if instr.source_line > max_line { max_line = instr.source_line; }
                }
            }
        }
        if min_line == u32::MAX {
            for ip in start_ip..=end_ip {
                if let Some(instr) = info.instructions.get(ip as usize) {
                    if instr.source_line > 0 {
                        if instr.source_line < min_line { min_line = instr.source_line; }
                        if instr.source_line > max_line { max_line = instr.source_line; }
                    }
                }
            }
        }
        
        let mut input_ip = None;
        for ip in start_ip..=end_ip {
            if let Some(instr) = info.instructions.get(ip as usize) {
                if instr.opcode == INPUT {
                    input_ip = Some(ip);
                    break;
                }
            }
        }
        
        if let Some(input_ip) = input_ip {
            let mut block_instructions_a = Vec::new();
            for ip in start_ip..input_ip {
                if let Some(instr) = info.instructions.get(ip as usize) {
                    block_instructions_a.push(instr.clone());
                }
            }
            
            let mut block_instructions_b = Vec::new();
            for ip in input_ip..=end_ip {
                if let Some(instr) = info.instructions.get(ip as usize) {
                    block_instructions_b.push(instr.clone());
                }
            }
            
            let start_col = 1;
            
            let full_source = get_source_range(source, min_line, start_col, max_line, 9999);
            let parts: Vec<&str> = full_source.split("<-").collect();
            let (left_part, right_part) = if parts.len() == 2 {
                (parts[0].trim().to_string(), parts[1].trim().to_string())
            } else {
                (full_source.clone(), String::new())
            };
            
            nodes_map.insert(start_ip, CFGNode {
                id: start_ip,
                start_ip,
                end_ip: input_ip - 1,
                instructions: block_instructions_a,
                source_text: right_part,
                node_type: "output".to_string(),
                called_function_ref: None,
            });
            
            nodes_map.insert(input_ip, CFGNode {
                id: input_ip,
                start_ip: input_ip,
                end_ip,
                instructions: block_instructions_b,
                source_text: left_part,
                node_type: "input".to_string(),
                called_function_ref: None,
            });
        } else {
            let mut block_instructions = Vec::new();
            let mut has_input = false;
            let mut has_call = false;
            let mut has_for_init = false;
            let mut has_for_iter = false;
            let mut has_condition = false;
            let mut has_terminal = false;
            let mut has_return = false;
            let mut has_bare_return = false;
            let mut print_call_info = None;
            let mut called_function_ref = None;
            
            for ip in start_ip..=end_ip {
                if let Some(instr) = info.instructions.get(ip as usize) {
                    block_instructions.push(instr.clone());
                    match instr.opcode {
                        INPUT => has_input = true,
                        CALL => {
                            has_call = true;
                            if let Some(argc) = is_print_call(info, ip) {
                                print_call_info = Some((ip, argc));
                            } else {
                                let mut found_ref = None;
                                let argc = instr.operand;
                                
                                /*
                                 * Direct stack position lookup
                                 * ------------------------------
                                 * First try direct stack position: ip - argc - 1
                                 */
                                if ip >= argc + 1 {
                                    let target_ip = ip - argc - 1;
                                    if let Some(target_instr) = info.instructions.get(target_ip as usize) {
                                        if target_instr.opcode == GET_GLOBAL {
                                            found_ref = global_functions.get(&target_instr.operand).copied();
                                        } else if target_instr.opcode == GET_LOCAL {
                                            found_ref = local_functions.get(&target_instr.operand).copied();
                                        } else if target_instr.opcode == LOAD_CONST {
                                            if let Some(c) = info.constants.get(target_instr.operand as usize) {
                                                if c.const_type == "function" || c.const_type == "class" || c.const_type == "object" {
                                                    found_ref = c.raw;
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                /*
                                 * Fallback lookup
                                 * ------------------------------
                                 * Search backwards for the closest resolvable function reference
                                 */
                                if found_ref.is_none() {
                                    for prev_ip in (start_ip..ip).rev() {
                                        if let Some(prev_instr) = info.instructions.get(prev_ip as usize) {
                                            if prev_instr.opcode == GET_GLOBAL {
                                                if let Some(&r) = global_functions.get(&prev_instr.operand) {
                                                    found_ref = Some(r);
                                                    break;
                                                }
                                            } else if prev_instr.opcode == GET_LOCAL {
                                                if let Some(&r) = local_functions.get(&prev_instr.operand) {
                                                    found_ref = Some(r);
                                                    break;
                                                }
                                            } else if prev_instr.opcode == LOAD_CONST {
                                                if let Some(c) = info.constants.get(prev_instr.operand as usize) {
                                                    if c.const_type == "function" || c.const_type == "class" || c.const_type == "object" {
                                                        found_ref = c.raw;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                called_function_ref = found_ref;
                            }
                        }
                        FOR_INIT => has_for_init = true,
                        FOR_ITER | FOR_ITER_ARRAY | FOR_ITER_RANGE | FOR_ITER_STRING => has_for_iter = true,
                        JUMP_IF | JUMP_IF_NOT => has_condition = true,
                        /*
                         * Return node classification
                         * ------------------------------
                         * RETURN_REF/RETURN_SELF: visible return node (returns a value)
                         * RETURN (bare): pure flow terminator — invisible, all paths lead to Fin
                         */
                        RETURN_REF | RETURN_SELF => {
                            has_terminal = true;
                            has_return = true;
                        }
                        RETURN => {
                            has_terminal = true;
                            has_bare_return = true;
                        }
                        HALT => has_terminal = true,
                        _ => {}
                    }
                }
            }
            
            let mut node_type = "process".to_string();
            if has_bare_return {
                node_type = "bare_return".to_string();
            } else if has_return {
                node_type = "return".to_string();
            } else if has_terminal {
                node_type = "terminal".to_string();
            } else if has_for_iter {
                node_type = "preparation".to_string();
            } else if has_condition {
                node_type = "condition".to_string();
            } else if has_input {
                node_type = "input".to_string();
            } else if has_for_init {
                node_type = "preparation".to_string();
            } else if has_call {
                node_type = "predefined_process".to_string();
            }
            
            let mut source_text = String::new();

            if has_return {
                if min_line <= max_line && min_line > 0 {
                    let start_col = 1;
                    source_text = get_source_range(source, min_line, start_col, max_line, 9999);
                }
                if source_text.is_empty() {
                    source_text = "Return".to_string();
                }
            } else if let Some((call_ip, _argc)) = print_call_info {
                node_type = "output".to_string();
                let mut print_load_ip = None;
                for prev_ip in (start_ip..call_ip).rev() {
                    if let Some(instr) = info.instructions.get(prev_ip as usize) {
                        if instr.opcode == LOAD_CONST {
                            if let Some(const_info) = info.constants.get(instr.operand as usize) {
                                if const_info.raw == get_print_ref().map(|r| r.raw()) {
                                    print_load_ip = Some(prev_ip);
                                    break;
                                }
                            }
                        }
                    }
                }
                
                let start_instr = print_load_ip
                    .and_then(|ip| info.instructions.get(ip as usize))
                    .or_else(|| info.instructions.get(start_ip as usize));
                
                if let Some(instr) = start_instr {
                    let start_line = instr.source_line;
                    let start_col = 1;
                    if start_line > 0 && max_line > 0 {
                        source_text = get_source_range(source, start_line, start_col, max_line, 9999);
                    }
                }
            } else if has_call {
                if min_line <= max_line && min_line > 0 {
                    let start_col = 1;
                    source_text = get_source_range(source, min_line, start_col, max_line, 9999);
                }
            }
            
            if source_text.is_empty() {
                if min_line <= max_line && min_line > 0 {
                    let start_col = 1;
                    source_text = get_source_range(source, min_line, start_col, max_line, 9999);
                }
            }
            
            if source_text.is_empty() {
                source_text = block_instructions.iter().map(|i| i.opcode_name.as_str()).collect::<Vec<&str>>().join(" ");
                if source_text.len() > 30 {
                    source_text.truncate(27);
                    source_text.push_str("...");
                }
            }
            
            nodes_map.insert(start_ip, CFGNode {
                id: start_ip,
                start_ip,
                end_ip,
                instructions: block_instructions,
                source_text,
                node_type,
                called_function_ref,
            });
        }
    }
    
    let mut raw_edges = Vec::new();
    let nodes_keys: Vec<u32> = nodes_map.keys().cloned().collect();
    
    for (&start_ip, node) in &nodes_map {
        let mut has_unconditional_jump = false;
        let mut has_return = false;
        let mut condition_instr: Option<&InstructionInfo> = None;
        
        for instr in &node.instructions {
            match instr.opcode {
                JUMP => {
                    has_unconditional_jump = true;
                    let target = find_leader(&nodes_keys, instr.operand);
                    raw_edges.push(CFGEdge {
                        from: start_ip,
                        to: target,
                        label: String::new(),
                    });
                }
                JUMP_IF | JUMP_IF_NOT => {
                    condition_instr = Some(instr);
                    let truthy_label = if instr.opcode == JUMP_IF { "true" } else { "false" };
                    let target = find_leader(&nodes_keys, instr.operand);
                    raw_edges.push(CFGEdge {
                        from: start_ip,
                        to: target,
                        label: String::from(truthy_label),
                    });
                }
                RETURN | RETURN_REF | RETURN_SELF | HALT => {
                    has_return = true;
                }
                _ => {}
            }
        }
        
        if !has_unconditional_jump && !has_return {
            if let Some(next_start) = get_next_leader(&nodes_keys, start_ip) {
                let label = if let Some(cond_instr) = condition_instr {
                    if cond_instr.opcode == JUMP_IF { "false" } else { "true" }
                } else {
                    ""
                };
                
                raw_edges.push(CFGEdge {
                    from: start_ip,
                    to: next_start,
                    label: String::from(label),
                });
            }
        }
    }
    
    let mut line_to_ids: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for (&id, node) in &nodes_map {
        let mut first_line = 0;
        for instr in &node.instructions {
            if instr.source_line > 0 {
                first_line = instr.source_line;
                break;
            }
        }
        if first_line > 0 {
            line_to_ids.entry(first_line).or_default().push(id);
        }
    }
    
    let mut primary_ids = BTreeSet::new();
    let mut bypassed_map: BTreeMap<u32, u32> = BTreeMap::new();
    
    for (&_line, ids) in &line_to_ids {
        let has_loop_iter = ids.iter().any(|&id| {
            if let Some(node) = nodes_map.get(&id) {
                node.instructions.iter().any(|instr| {
                    matches!(instr.opcode, FOR_ITER | FOR_ITER_ARRAY | FOR_ITER_RANGE | FOR_ITER_STRING)
                })
            } else {
                false
            }
        });
        
        let mut meaningful_nodes = Vec::new();
        let mut helper_nodes = Vec::new();
        for &id in ids {
            if let Some(node) = nodes_map.get(&id) {
                let is_helper = if has_loop_iter {
                    let is_this_loop_iter = node.instructions.iter().any(|instr| {
                        matches!(instr.opcode, FOR_ITER | FOR_ITER_ARRAY | FOR_ITER_RANGE | FOR_ITER_STRING)
                    });
                    !is_this_loop_iter && (node.node_type == "process" || node.node_type == "preparation")
                } else {
                    is_helper_node(node)
                };
                
                if is_helper {
                    helper_nodes.push(id);
                } else {
                    meaningful_nodes.push(id);
                }
            }
        }
        
        if meaningful_nodes.is_empty() {
            for &id in ids {
                bypassed_map.insert(id, 0);
            }
        } else {
            for &id in &meaningful_nodes {
                primary_ids.insert(id);
            }
            for &id in &helper_nodes {
                if has_loop_iter {
                    let iter_node_id = ids.iter().find(|&&id| {
                        if let Some(node) = nodes_map.get(&id) {
                            node.instructions.iter().any(|instr| {
                                matches!(instr.opcode, FOR_ITER | FOR_ITER_ARRAY | FOR_ITER_RANGE | FOR_ITER_STRING)
                            })
                        } else {
                            false
                        }
                    }).copied().unwrap_or(meaningful_nodes[0]);
                    bypassed_map.insert(id, iter_node_id);
                } else {
                    bypassed_map.insert(id, 0);
                }
            }
        }
    }
    
    for &id in nodes_map.keys() {
        let mut has_line = false;
        if let Some(node) = nodes_map.get(&id) {
            for instr in &node.instructions {
                if instr.source_line > 0 {
                    has_line = true;
                    break;
                }
            }
        }
        if !has_line {
            bypassed_map.insert(id, 0);
        }
    }
    
    for func in &dyn_functions {
        bypassed_map.insert(func.end_ip + 1, 0);
    }
    
    for &bypassed_id in bypassed_map.keys().cloned().collect::<Vec<u32>>().iter() {
        let mut curr = bypassed_id;
        let mut visited = BTreeSet::new();
        visited.insert(curr);
        
        loop {
            let target = find_raw_successor(curr, &raw_edges);
            if let Some(t) = target {
                if bypassed_map.contains_key(&t) {
                    if visited.contains(&t) {
                        bypassed_map.insert(bypassed_id, t);
                        break;
                    }
                    visited.insert(t);
                    curr = t;
                } else {
                    bypassed_map.insert(bypassed_id, t);
                    break;
                }
            } else {
                bypassed_map.remove(&bypassed_id);
                break;
            }
        }
    }
    
    let phantom_nodes: BTreeSet<u32> = nodes_map.iter()
        .filter(|(_, node)| node.instructions.is_empty() && node.source_text.is_empty())
        .map(|(&id, _)| id)
        .collect();

    /*
     * Compiler-injected registration blocks
     * ------------------------------
     * LOAD_CONST + SET_GLOBAL + POP after each function body are never meaningful flow nodes.
     * Collect them explicitly so they are always excluded from the rendered graph — even when
     * the bypass chain resolution fails to route around them (such as when they are the last
     * thing in the bytecode and have no raw successor).
     */
    let registration_block_ids: BTreeSet<u32> = dyn_functions.iter()
        .map(|f| f.end_ip + 1)
        .collect();

    /*
     * Bare-return node IDs
     * ------------------------------
     * Invisible flow terminators that must never be rendered.
     */
    let bare_return_ids: BTreeSet<u32> = nodes_map.iter()
        .filter(|(_, n)| n.node_type == "bare_return")
        .map(|(&id, _)| id)
        .collect();

    let mut all_final_nodes = Vec::new();
    for (&id, node) in &nodes_map {
        if primary_ids.contains(&id)
            && !bypassed_map.contains_key(&id)
            && !phantom_nodes.contains(&id)
            && node.node_type != "bare_return"
            && !registration_block_ids.contains(&id)
        {
            all_final_nodes.push(node.clone());
        }
    }

    let mut unique_edges = BTreeSet::new();
    let mut all_final_edges = Vec::new();
    for edge in &raw_edges {
        if bypassed_map.contains_key(&edge.from) {
            continue;
        }
        /*
         * Skip edges from invisible nodes
         * ------------------------------
         * Bare-return or registration-block sources should be skipped.
         */
        if nodes_map.get(&edge.from).map_or(false, |n| n.node_type == "bare_return") {
            continue;
        }
        if registration_block_ids.contains(&edge.from) {
            continue;
        }

        let mut final_to = edge.to;
        let mut visited = BTreeSet::new();
        while let Some(&succ) = bypassed_map.get(&final_to) {
            if visited.contains(&succ) {
                break;
            }
            visited.insert(succ);
            final_to = succ;
        }

        /*
         * Invisible node resolution
         * ------------------------------
         * If the resolved target is an invisible node (bare_return or registration block),
         * treat it as a flow exit so the subgraph exit-node detection works correctly and
         * connects the predecessor to "Fin" instead of leaving it dangling.
         */
        if bare_return_ids.contains(&final_to) || registration_block_ids.contains(&final_to) {
            final_to = u32::MAX - 1;
        }

        if edge.from != final_to {
            if phantom_nodes.contains(&final_to) {
                let has_succ = raw_edges.iter().any(|e| e.from == final_to);
                if !has_succ {
                    final_to = u32::MAX - 1;
                }
            }

            if unique_edges.insert((edge.from, final_to)) {
                all_final_edges.push(CFGEdge {
                    from: edge.from,
                    to: final_to,
                    label: edge.label.clone(),
                });
            }
        }
    }

    /*
     * Partition nodes and edges into functions
     * ------------------------------
     * Separate function-specific nodes/edges from main graph
     */
    let mut subgraphs = Vec::new();
    let mut main_nodes = Vec::new();
    let mut main_edges = Vec::new();

    for func in &dyn_functions {
        let mut func_nodes = Vec::new();
        for node in &all_final_nodes {
            if node.start_ip >= func.start_ip && node.end_ip <= func.end_ip {
                func_nodes.push(node.clone());
            }
        }

        let mut func_edges = Vec::new();
        for edge in &all_final_edges {
            if edge.from >= func.start_ip && edge.from <= func.end_ip {
                func_edges.push(edge.clone());
            }
        }

        let mut name = format!("Function_{}", func.index);
        let function_ref = func.function_ref;

        /*
         * Function name extraction
         * ------------------------------
         * Find the JUMP that skips this function to get its name from source
         */
        let mut jump_instr_line = 0;
        if let Some(instr) = info.instructions.get(func.jump_ip as usize) {
            jump_instr_line = instr.source_line;
        }

        if jump_instr_line > 0 {
            name = get_source_range(source, jump_instr_line, 1, jump_instr_line, 9999).trim().to_string();
        }

        /*
         * Subgraph Start node
         * ------------------------------
         * Add local Start node for subgraph with unique offset
         */
        let start_id = func.start_ip + 1000000;
        func_nodes.insert(0, CFGNode {
            id: start_id,
            start_ip: 0,
            end_ip: 0,
            instructions: Vec::new(),
            source_text: "Start".to_string(),
            node_type: "terminal".to_string(),
            called_function_ref: None,
        });

        /*
         * Start node linkage
         * ------------------------------
         * Link Start node to first node in function
         */
        if let Some(first_node) = func_nodes.get(1) {
            func_edges.insert(0, CFGEdge {
                from: start_id,
                to: first_node.id,
                label: String::new(),
            });
        }

        /*
         * Exit node detection
         * ------------------------------
         * Find exit nodes (nodes with no outgoing edges, except ones pointing to global end MAX-1).
         * Also detect any internal edges already pointing to MAX-1 (such as the false branch of a
         * conditional whose target was a bare RETURN, redirected to MAX-1 during edge resolution).
         */
        let has_internal_exit_edges = func_edges.iter().any(|e| e.to == u32::MAX - 1);
        let exit_nodes: Vec<u32> = func_nodes.iter()
            .map(|n| n.id)
            .filter(|&id| id != start_id)
            .filter(|&id| !func_edges.iter().any(|e| e.from == id && e.to != u32::MAX - 1))
            .collect();

        if !exit_nodes.is_empty() || has_internal_exit_edges {
            let end_id = func.start_ip + 2000000; // Unique offset for subgraph end
            func_nodes.push(CFGNode {
                id: end_id,
                start_ip: 0,
                end_ip: 0,
                instructions: Vec::new(),
                source_text: "Fin".to_string(),
                node_type: "terminal".to_string(),
                called_function_ref: None,
            });

            for &from_id in &exit_nodes {
                // Remove edge to MAX-1 if it exists
                func_edges.retain(|e| !(e.from == from_id && e.to == u32::MAX - 1));

                func_edges.push(CFGEdge {
                    from: from_id,
                    to: end_id,
                    label: String::new(),
                });
            }

            /*
             * MAX-1 sentinel redirection
             * ------------------------------
             * Redirect all remaining MAX-1 sentinels inside this subgraph to its own Fin.
             * This handles cases like the false-branch of `if` leading to a bare RETURN:
             * those edges were already converted to MAX-1 during all_final_edges resolution
             * but still need to point to the local Fin, not the global End.
             */
            for edge in func_edges.iter_mut() {
                if edge.to == u32::MAX - 1 {
                    edge.to = end_id;
                }
            }
        }

        subgraphs.push(CFGSubgraph {
            id: format!("subgraph_{}", func.index),
            name,
            function_ref,
            nodes: func_nodes,
            edges: func_edges,
        });
    }

    /*
     * Filter out function nodes from main graph
     * ------------------------------
     * Separate function-specific nodes/edges from main graph
     */
    for node in &all_final_nodes {
        let mut is_in_func = false;
        for func in &dyn_functions {
            if node.start_ip >= func.start_ip && node.end_ip <= func.end_ip {
                is_in_func = true;
                break;
            }
        }
        if !is_in_func {
            main_nodes.push(node.clone());
        }
    }

    for edge in &all_final_edges {
        let mut is_in_func = false;
        for func in &dyn_functions {
            if edge.from >= func.start_ip && edge.from <= func.end_ip {
                is_in_func = true;
                break;
            }
        }
        if !is_in_func {
            main_edges.push(edge.clone());
        }
    }

    let mut start_target = 0;
    let mut visited = BTreeSet::new();
    while let Some(&succ) = bypassed_map.get(&start_target) {
        if visited.contains(&succ) {
            break;
        }
        visited.insert(succ);
        start_target = succ;
    }

    main_nodes.insert(0, CFGNode {
        id: u32::MAX,
        start_ip: 0,
        end_ip: 0,
        instructions: Vec::new(),
        source_text: "Start".to_string(),
        node_type: "terminal".to_string(),
        called_function_ref: None,
    });

    main_edges.insert(0, CFGEdge {
        from: u32::MAX,
        to: start_target,
        label: String::new(),
    });

    let exit_nodes: Vec<u32> = main_nodes.iter()
        .map(|n| n.id)
        .filter(|&id| id != u32::MAX && id != u32::MAX - 1)
        .filter(|&id| !main_edges.iter().any(|e| e.from == id && e.to != u32::MAX - 1))
        .collect();

    if !exit_nodes.is_empty() || main_edges.iter().any(|e| e.to == u32::MAX - 1) {
        let end_id = u32::MAX - 1;
        main_nodes.push(CFGNode {
            id: end_id,
            start_ip: 0,
            end_ip: 0,
            instructions: Vec::new(),
            source_text: "End".to_string(),
            node_type: "terminal".to_string(),
            called_function_ref: None,
        });

        for &from_id in &exit_nodes {
            if !main_edges.iter().any(|e| e.from == from_id && e.to == end_id) {
                main_edges.push(CFGEdge {
                    from: from_id,
                    to: end_id,
                    label: String::new(),
                });
            }
        }
    }

    ControlFlowGraph { 
        nodes: main_nodes, 
        edges: main_edges, 
        subgraphs 
    }
}
