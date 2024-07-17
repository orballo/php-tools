use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "grammar/base.pest"]
pub struct CSVParser {}

pub fn main() {
    let file_content = fs::read_to_string("packages/parser/src/fixtures/hello-world.php")
        .expect("Failed to read the file");
    let file = CSVParser::parse(Rule::FILE, &file_content).expect("Failed to parse");

    eprintln!("file = {:#?}", file);
}
