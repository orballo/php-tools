use parser::{PHPParser, PHPRule};
use pest::Parser;
use std::fs;
use std::path::Path;

pub fn main() {
    let file_content = fs::read_to_string("packages/parser/src/fixtures/hello-world.php")
        .expect("Failed to read the file\n");
    let file = PHPParser::parse(PHPRule::FILE, &file_content).expect("Failed to parse\n");
    let pretty_string = format!("{:#?}", file);

    let output_path = Path::new("output");

    fs::create_dir_all(output_path).expect("Failed to create output path\n");

    fs::write(output_path.join("pairs.txt"), pretty_string).expect("Failed to write `pairs.txt`\n");
}
