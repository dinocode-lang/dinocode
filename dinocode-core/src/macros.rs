// ═══════════════════════════════════════════════════════════
//  DinoCode – Language and Interpreter
//  
//  File:       src/macros.rs
//  Desc:       Declarative macros
//  
//  Author:     Ismael Quiroz
//  Copyright: (C) 2025-2026 Ismael Quiroz (@BlassGO)
//  License:    Apache License 2.0 (See 'LICENSE' file for full terms)
// ═══════════════════════════════════════════════════════════

#[macro_export]
macro_rules! register_module {
    (
        name: $init_name:ident
        $(, functions: [$($func:ident),* $(,)?])?
        $(, classes: [$($class:ident),* $(,)?])?
    ) => {
        pub fn $init_name() {
            paste::paste! {
                $($(
                    [<_dinof_init_ $func>]();
                )*)?
                $($(
                    [<_dinoclass_init_ $class>]();
                    [<_dinoclass_init_methods_ $class>]();
                )*)?
            }
        }
    }
}
