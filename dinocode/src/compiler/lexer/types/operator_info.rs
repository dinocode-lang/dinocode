// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/compiler/lexer/types/operator_follow.rs
//  Desc:       Operator follow table for lexical analysis
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

pub const OPERATOR_INFO: [(&[u8], usize); 128] = {
    let mut t: [(&[u8], usize); 128] = [(b"", 0); 128];
    t[b'<' as usize] = (b"-<=", 2);
    t[b'=' as usize] = (b"=~", 2);
    t[b'!' as usize] = (b"=~", 2);
    t[b'|' as usize] = (b"|=", 2);
    t[b'&' as usize] = (b"&=", 2);
    t[b'-' as usize] = (b"-=.", 2);
    t[b'+' as usize] = (b"+=", 2);
    t[b'>' as usize] = (b">=", 2);
    t[b'*' as usize] = (b"*=", 2);
    t[b'/' as usize] = (b"/=", 2);
    t[b'^' as usize] = (b"^=", 2);
    t[b'%' as usize] = (b"%=", 2);
    t[b'.' as usize] = (b".=\\", 3);
    t[b'~' as usize] = (b"", 1);
    t[b'?' as usize] = (b"", 1);
    t[b':' as usize] = (b":", 2);
    t
};