use pest::Parser;
use pest_derive::Parser;
use serde_json::{json, Value};
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

    let pretty_string = format!("{:#?}", file);
    let json: Value = json!(file);
    let pretty_json = serde_json::to_string_pretty(&json).expect("failed to strngify json");

    _ = fs::write("pest-output.txt", pretty_string);
    _ = fs::write("output-output.json", pretty_json);
}
