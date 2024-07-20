use pest::Parser;
use pest_derive::Parser;
use std::fs::{self, create_dir_all};
use std::path::Path;

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
    let pretty_string = format!("{:#?}", file);

    let output_path = Path::new("output");

    create_dir_all(output_path).expect("Failed to create output path");

    fs::write(output_path.join("pairs.txt"), pretty_string).expect("Failed to write `pairs.txt`");
}
