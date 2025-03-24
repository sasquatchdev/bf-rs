pub enum ParsingResult {
    Invalid,
    Compile { input: String, output: String, compiler: String },
    Interpret { input: String },
}

/*
    bf-rs [-c <input> <output> <compiler>] [-i <input>]
*/
pub fn parse() -> ParsingResult {
    let args: Vec<String> = std::env::args().skip(1).collect();
    
    // "-c <input> <output> <compiler>" => 4 OR "-i <input>" => 2
    if args.len() == 0 {
        return ParsingResult::Invalid;
    }

    if args[0] == "-c" && args.len() == 4 {
        // "-c <input> <output> <compiler>"
        return ParsingResult::Compile {
            input: args[1].clone(),
            output: args[2].clone(),
            compiler: args[3].clone(),
        };
    } else if args[0] == "-i" && args.len() == 2 {
        // "-i <input>"
        return ParsingResult::Interpret {
            input: args[1].clone(),
        };
    }

    ParsingResult::Invalid
}

pub fn usage() {
    println!("Usage: bf-rs [-c <input> <output> <compiler>] [-i <input>]");
}
