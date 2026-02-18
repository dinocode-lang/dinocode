# 🦖 DinoCode

[![Español](https://img.shields.io/badge/Leer%20en-Español-red?style=flat-square)](./README.md)

**DinoCode** is a programming language based on **programmer intent inference**, specifically designed to eliminate syntactic friction through the **Golden Rule** paradigm.

> [!IMPORTANT]
> DinoCode is the technical core of a thesis project. It is currently distributed in **compiled binaries** for usability testing. The source code remains private until the degree defense, where it is intended to be released.
---

## The Golden Rule

### Main pillars:
- **Intent inference:** The DinoCode engine interprets the code topography to deduce the logical structure.
- **Golden Rule:** Syntax is an emergent property of the programmer's intent, not an imposition of the compiler.
- **Optimized architecture:** The current implementation is written in Rust and translates source code directly to a virtual machine with linear complexity $O(n)$, without going through a conventional AST (Abstract Syntax Tree).

---

## How to try DinoCode

**Option 1: DinoIDE**

[![Download DinoIDE](https://img.shields.io/badge/DinoIDE-for%20Windows-008080?style=for-the-badge&logo=windows)](https://github.com/BlassGO/DinoIDE)

**Option 2: Command line (Linux/Windows)**
1. **Download the binary:** Go to the [Releases](https://github.com/dinocode-lang/dinocode/releases) section.
2. **Run a script**
   ```bash
    ./dinocode program.dino
   ```

## Your opinion matters!

If you've already tried the language, I'd love to know about your experience. Help me by filling out this survey for language usability validation, it would help a lot:

 **[DinoCode Usability Survey](https://forms.gle/3XVGrRxk1mr3Hxd98)**

### Syntax Example

```dinocode

:suma a b
    return a + b

:main args
    print "Hello world!"

    x = 10
    y = 1.5

    print [   # New list

        # Sum
        x+y

        # Multiplication
        x*y
    ]
    
    for arg in args
        print arg

```

## Other examples

[![Go to Benchmark Tests](https://img.shields.io/badge/Go%20to-Benchmark%20Tests-green?style=flat-square)](./examples/3_advanced/4_benchmarking.dino)

1. [`examples/`](./examples/)
   1. [`Golden Rule`](./examples/1_golden_rule/)
      1. [`Introduction`](./examples/1_golden_rule/1_introduction)
      2. [`Operational Continuity`](./examples/1_golden_rule/2_operational_continuity.dino)
      3. [`Intent Inference`](./examples/1_golden_rule/3_intent_inference.dino)
      4. [`Considerations`](./examples/1_golden_rule/4_considerations.dino)
      5. [`In Practice`](./examples/1_golden_rule/5_in_practice.dino)
   2. [`Basic Syntax`](./examples/2_basic/)
      1. [`Arithmetic`](./examples/2_basic/1_arithmetic.dino)
      2. [`Flexible Syntax`](./examples/2_basic/2_flexible_syntax.dino)
      3. [`Interpolation`](./examples/2_basic/3_interpolation.dino)
      4. [`Strings`](./examples/2_basic/4_strings.dino)
      5. [`Flow Control`](./examples/2_basic/5_flow_control.dino)
      6. [`Functions`](./examples/2_basic/6_functions.dino)
      7. [`Main Function`](./examples/2_basic/7_the_main_function.dino)
      8. [`Arrays`](./examples/2_basic/8_arrays.dino)
      9. [`Objects`](./examples/2_basic/9_objects.dino)
      10. [`New Objects`](./examples/2_basic/10_new_objects.dino)
      11. [`Dollar Call`](./examples/2_basic/11_dollar_call.dino)
      12. [`Templates`](./examples/2_basic/12_templates.dino)
   3. [`Advanced`](./examples/3_advanced/)
      1. [`BigIntegers`](./examples/3_advanced/1_bigintegers.dino)
      2. [`Other Numbers`](./examples/3_advanced/2_other_numbers.dino)
      3. [`Array Methods`](./examples/3_advanced/3_array_methods.dino)
      4. [`Object Methods`](./examples/3_advanced/4_object_methods.dino)
      5. [`Benchmarking`](./examples/3_advanced/5_benchmarking.dino)
      6. [`Calculator`](./examples/3_advanced/calculator.dino)
      7. [`Console Library`](./examples/3_advanced/console_library.dino)
      8. [`Fibonacci`](./examples/3_advanced/fibonacci.dino)

---

## ⚖️ Authorship and license

DinoCode is an original technological work. The technical architecture, the inference engine design, and the complete implementation in Rust are the exclusive property of the author. No third party has participated in the technical research, the engine logic, or its source code.
* **Author:** Ismael Quiroz ([@BlassGO](https://github.com/BlassGO))

### Terms of use

Currently, the project is distributed under the **Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International (CC BY-NC-ND 4.0)** license.

> [!NOTE]
> This restrictive license remains in effect while the project goes through its academic validation and thesis substantiation phase. The author intends to transition to a **more permissive license** once the graduation process is completed and the source code is released.

For more details, consult the [LICENSE](./LICENSE) file in this repository.

---
