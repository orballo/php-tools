use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar/base.pest"]
#[grammar = "grammar/lexical/keywords.pest"]
#[grammar = "grammar/lexical/operators.pest"]
#[grammar = "grammar/lexical/punctuators.pest"]
#[grammar = "grammar/lexical/names.pest"]
#[grammar = "grammar/lexical/literals.pest"]
pub struct PHPParser {}

pub fn main() {
    let file_content = fs::read_to_string("packages/parser/src/fixtures/hello-world.php")
        .expect("Failed to read the file");
    let file = PHPParser::parse(Rule::FILE, &file_content).expect("Failed to parse");

    eprintln!("file = {:#?}", file);
}
