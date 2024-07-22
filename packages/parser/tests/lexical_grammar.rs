use insta::{assert_yaml_snapshot, glob, with_settings};
use parser::{PHPParser, PHPRule};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use serde::Serialize;
use std::fs;

// @TODO: Use regex_lite to extract `@case_name` from comments
// Find a way to know that the comment and the case are related.
// They need to be in consecutive lines.

fn extract_rules(pairs: Pairs<PHPRule>, rule: PHPRule) -> Vec<Pair<PHPRule>> {
    let mut extracted_rules = Vec::new();

    for pair in pairs {
        if pair.as_rule() == PHPRule::COMMENT {
            extracted_rules.push(pair.clone());
        }

        if pair.as_rule() == rule {
            extracted_rules.push(pair.clone());
        }

        // Recursively process nested pairs
        let inner_pairs = pair.into_inner();
        let mut inner_extracted = extract_rules(inner_pairs, rule);
        extracted_rules.append(&mut inner_extracted);
    }

    extracted_rules
}

// #[test]
// fn lexical_grammar_snapshots() {
//     let mut settings = Settings::clone_current();
//     settings.set_prepend_module_to_snapshot(false);

//     settings.bind(|| {
//         glob!("fixtures/script.php", |path| {
//             let file = fs::read_to_string(path).unwrap();
//             let mut parsed_file = PHPParser::parse(PHPRule::SCRIPT, &file)
//                 .expect("Something went wrong parsing the file");

//             assert_debug_snapshot!(parsed_file.next().unwrap());
//         })
//     });
// }

#[derive(Serialize)]
struct SingleQuotedString {
    rule: String,
    content: String,
}

#[test]
fn strings_single_quoted_literals() {
    glob!("fixtures/strings_single_quoted.php", |path| {
        let file = fs::read_to_string(path).unwrap();
        let pairs = PHPParser::parse(PHPRule::SCRIPT, &file)
            .expect("Something went wrong parsing the file");
        let pairs = extract_rules(pairs, PHPRule::SINGLE_QUOTED_STRING_LITERAL);
        // eprintln!("pairs = {:#?}", pairs);
        // let nowdoc_strings: Vec<SingleQuotedString> = pairs
        //     .into_iter()
        //     .map(|pair| {
        //         let content = pair.as_str().to_string();

        //         SingleQuotedString {
        //             rule: "Single Quoted String".to_string(),
        //             content,
        //         }
        //     })
        //     .collect();

        with_settings!({prepend_module_to_snapshot => false}, {
            assert_yaml_snapshot!(pairs);
        })
    });
}

// #[derive(Serialize)]
// struct NowdocString {
//     rule: String,
//     start_identifier: String,
//     content: String,
//     end_identifier: String,
// }

// #[test]
// fn strings_nowdoc_literals() {
//     glob!("fixtures/strings_nowdoc.php", |path| {
//         let file = fs::read_to_string(path).unwrap();
//         let pairs = PHPParser::parse(PHPRule::SCRIPT, &file)
//             .expect("Something went wrong parsing the file");
//         let pairs = extract_rules(pairs, PHPRule::NOWDOC_STRING_LITERAL);
//         let nowdoc_strings: Vec<NowdocString> = pairs
//             .into_iter()
//             .map(|pair| {
//                 let mut children = pair.into_inner();
//                 let start_identifier = children.next().map_or("", |p| p.as_str()).to_string();
//                 let content = children.next().map_or("", |p| p.as_str()).to_string();
//                 let end_identifier = children.next().map_or("", |p| p.as_str()).to_string();

//                 NowdocString {
//                     rule: "Nowdoc String".to_string(),
//                     start_identifier,
//                     content,
//                     end_identifier,
//                 }
//             })
//             .collect();

//         with_settings!({prepend_module_to_snapshot => false}, {
//             assert_yaml_snapshot!(nowdoc_strings);
//         })
//     });
// }
