use insta::{assert_debug_snapshot, glob};
use parser::{PHPParser, PHPRule};
use pest::Parser;
use std::fs;

#[test]
fn lexical_grammar_snapshots() {
    let mut settings = insta::Settings::clone_current();
    settings.set_prepend_module_to_snapshot(false);

    settings.bind(|| {
        glob!("fixtures/*.php", |path| {
            let file = fs::read_to_string(path).unwrap();
            let mut parsed_file = PHPParser::parse(PHPRule::SCRIPT, &file)
                .expect("Something went wrong parsing the file");

            assert_debug_snapshot!(parsed_file.next().unwrap());
        })
    });
}
