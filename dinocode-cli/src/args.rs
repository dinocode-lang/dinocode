#[derive(Debug)]
pub struct Args {
    pub file: Option<String>,
    pub version: bool,
    pub help: bool,
    pub main_args: Vec<String>,
    pub show_tokens: bool,
    pub show_bytecode: bool,
}

impl Args {
    pub fn parse() -> Self {
        let mut args = std::env::args().collect::<Vec<_>>();
        
        // Skip program name
        if args.len() > 1 {
            args.remove(0);
        } else {
            return Args::default();
        }
        
        let mut result = Args::default();
        let mut script_found = false;
        let mut i = 0;
        
        while i < args.len() {
            let arg = &args[i];
            
            if !script_found {
                match arg.as_str() {
                    "--version" => {
                        result.version = true;
                    }
                    "--help" => {
                        result.help = true;
                    }
                    "--tokens" => {
                        result.show_tokens = true;
                    }
                    "--bytecode" => {
                        result.show_bytecode = true;
                    }
                    _ if arg.starts_with("--") => {
                        eprintln!("Unknown option: {}", arg);
                        std::process::exit(1);
                    }
                    _ => {
                        result.file = Some(arg.clone());
                        script_found = true;
                    }
                }
            } else {
                result.main_args.push(arg.clone());
            }
            
            i += 1;
        }
        
        result
    }
    
    fn default() -> Self {
        Self {
            file: None,
            version: false,
            help: false,
            main_args: Vec::new(),
            show_tokens: false,
            show_bytecode: false,
        }
    }
}
