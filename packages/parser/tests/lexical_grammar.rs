use parser::{PHPParser, PHPRule};
use pest::Parser;
use std::fs;

macro_rules! fixture_path {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/", $fname)
    };
}

#[test]
fn matches_script() {
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

#[test]
fn matches_php_tags() {
    let variant_one = "<?php ?>";
    let variant_two = "<?= ?>";
    let variant_three = "<? ?>";

    let mut result =
        PHPParser::parse(PHPRule::SCRIPT_SECTION, variant_one).expect("Failed to parse PHP");

    let script_section = result.next().unwrap();

    assert_eq!(script_section.as_rule(), PHPRule::SCRIPT_SECTION);

    for (index, value) in script_section.into_inner().enumerate() {
        match index {
            0 => {
                assert_eq!(value.as_rule(), PHPRule::OPEN_TAG);
                assert_eq!(value.as_str(), "<?php");
            }
            1 => {
                assert_eq!(value.as_rule(), PHPRule::CLOSE_TAG);
                assert_eq!(value.as_str(), "?>");
            }
            _ => unreachable!(),
        }
    }

    let mut result =
        PHPParser::parse(PHPRule::SCRIPT_SECTION, variant_two).expect("Failed to parse PHP");

    let script_section = result.next().unwrap();

    assert_eq!(script_section.as_rule(), PHPRule::SCRIPT_SECTION);

    for (index, value) in script_section.into_inner().enumerate() {
        match index {
            0 => {
                assert_eq!(value.as_rule(), PHPRule::OPEN_TAG);
                assert_eq!(value.as_str(), "<?=");
            }
            1 => {
                assert_eq!(value.as_rule(), PHPRule::CLOSE_TAG);
                assert_eq!(value.as_str(), "?>");
            }
            _ => unreachable!(),
        }
    }

    let mut result =
        PHPParser::parse(PHPRule::SCRIPT_SECTION, variant_three).expect("Failed to parse PHP");

    let script_section = result.next().unwrap();

    assert_eq!(script_section.as_rule(), PHPRule::SCRIPT_SECTION);

    for (index, value) in script_section.into_inner().enumerate() {
        match index {
            0 => {
                assert_eq!(value.as_rule(), PHPRule::OPEN_TAG);
                assert_eq!(value.as_str(), "<?");
            }
            1 => {
                assert_eq!(value.as_rule(), PHPRule::CLOSE_TAG);
                assert_eq!(value.as_str(), "?>");
            }
            _ => unreachable!(),
        }
    }
}
