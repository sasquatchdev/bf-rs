pub const VALID: [char; 10] = ['<', '>', '+', '-', '.', ':', ',', ';', '[', ']'];

/// The `parse` module contains a very minimal
/// cli argument parse that simply checks for the
/// `-c` flag and a file path.
pub mod parse;

/// The `interpret` module contains the logic for
/// live-interpreting a Brainf*ck program.
pub mod interpret;

/// The `compile` module contains the logic for
/// compiling a Brainf*ck program into a different
/// language.
pub mod compile;

/// "pre-processes" the contents of a file
/// into a vector of valid Brainf*ck instructions.
pub fn process(contents: String) -> Vec<char> {
    contents.chars().filter(|c| VALID.contains(c)).collect()
}

fn main() {
    match parse::parse() {
        parse::ParsingResult::Invalid => parse::usage(),
        parse::ParsingResult::Interpret { input } => {
            let contents = std::fs::read_to_string(input).unwrap();
            let mem = interpret::interpret(contents).unwrap();
            let index = mem.iter().rposition(|x| *x != 0).unwrap();
            println!("{:?}", &mem[..=index]);
        },
        parse::ParsingResult::Compile { input, output, compiler } => {
            let contents = std::fs::read_to_string(input).unwrap();
            let compiled = match compiler.as_str() {
                "python" => compile::python::compile(contents),
                _ => unimplemented!("compiler not implemented")
            };
            std::fs::write(output, compiled).unwrap();
        }
    }
}
