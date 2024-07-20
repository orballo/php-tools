use parser::{PHPParser, PHPRule};
use pest::Parser;
use std::fs;

macro_rules! fixture_path {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/", $fname)
    };
}

#[test]
fn script_structure() {
    let unparsed_file = fs::read_to_string(fixture_path!("script_structure.php"))
        .expect("Something went wrong reading the file");
    let mut parsed_file = PHPParser::parse(PHPRule::FILE, &unparsed_file)
        .expect("Something went wrong parsing the file");

    let script = parsed_file.next().unwrap();

    assert_eq!(script.as_rule(), PHPRule::SCRIPT);

    for (index, value) in script.into_inner().enumerate() {
        match index {
            0 => {
                assert_eq!(value.as_rule(), PHPRule::TEXT);
                assert_eq!(value.as_str(), "Starting Random Text\n");
            }
            1 => {
                assert_eq!(value.as_rule(), PHPRule::SCRIPT_SECTION);
                assert_eq!(value.as_str(), "<?php ?>");
            }
            2 => {
                assert_eq!(value.as_rule(), PHPRule::TEXT);
                assert_eq!(value.as_str(), "In Between Random Text");
            }
            3 => {
                assert_eq!(value.as_rule(), PHPRule::SCRIPT_SECTION);
                assert_eq!(value.as_str(), "<?php ?>");
            }
            4 => {
                assert_eq!(value.as_rule(), PHPRule::TEXT);
                assert_eq!(value.as_str(), "\nEnding Random Text");
            }
            _ => unreachable!(),
        }
    }
}
