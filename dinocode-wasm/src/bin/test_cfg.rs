use dinocode_wasm::analysis::BytecodeAnalyzer;

fn main() {
    let source = "x = 10";
    let analyzer = BytecodeAnalyzer::from_source(source).unwrap();
    let cfg = analyzer.get_control_flow_graph().unwrap();
    println!("CFG FOR x = 10:\n{}", cfg);
}
