pub const VALID: [char; 10] = ['<', '>', '+', '-', '.', ':', ',', ';', '[', ']'];

/// The `parse` module contains a very minimal
/// cli argument parse that simply checks for the
/// `-c` flag and a file path.
pub mod parse;

/// The `interpret` module contains the logic for
/// live-interpreting a Brainf*ck program.
pub mod interpret;

fn main() {
    match parse::parse() {
        parse::ParsingResult::Invalid => parse::usage(),
        parse::ParsingResult::Interpret { input } => {
            let contents = std::fs::read_to_string(input).unwrap();
            let mem = interpret::interpret(contents).unwrap();
            let index = mem.iter().rposition(|x| *x != 0).unwrap();
            println!("{:?}", &mem[..=index]);
        },
        parse::ParsingResult::Compile { .. } => {
            unimplemented!("Compilation mode is not yet supported.")
        }
    }
}
