// mod common;

// use common::Extractor;
// use insta::{assert_yaml_snapshot, glob, with_settings};
// use parser::{PHPParser, PHPRule};
// use pest::Parser;
// use std::fs;

// #[test]
// fn single_quoted_string_literal() {
//     glob!("fixtures/single_quoted_string_literals.php", |path| {
//         let file = fs::read_to_string(path).unwrap();
//         let pairs = PHPParser::parse(PHPRule::SCRIPT, &file)
//             .expect("Something went wrong parsing the file");
//         let literals = Extractor::default()
//             .extract_nodes(pairs, PHPRule::SINGLE_QUOTED_STRING_LITERAL);

//         with_settings!({prepend_module_to_snapshot => false}, {
//             assert_yaml_snapshot!(literals);
//         })
//     });
// }

// #[test]
// fn double_quoted_string_literal() {
//     glob!("fixtures/double_quoted_string_literals.php", |path| {
//         let file = fs::read_to_string(path).unwrap();
//         let pairs = PHPParser::parse(PHPRule::SCRIPT, &file)
//             .expect("Something went wrong parsing the file");
//         let literals = Extractor::default()
//             .extract_nodes(pairs, PHPRule::DOUBLE_QUOTED_STRING_LITERAL);

//         with_settings!({prepend_module_to_snapshot => false}, {
//             insta::assert_yaml_snapshot!(literals);
//         })
//     });
// }

// #[test]
// fn heredoc_string_literal() {
//     glob!("fixtures/heredoc_string_literals.php", |path| {
//         let file = fs::read_to_string(path).unwrap();
//         let pairs = PHPParser::parse(PHPRule::SCRIPT, &file)
//             .expect("Something went wrong parsing the file");
//         let literals = Extractor::default()
//             .extract_nodes(pairs, PHPRule::HEREDOC_STRING_LITERAL);

//         with_settings!({prepend_module_to_snapshot => false}, {
//             insta::assert_yaml_snapshot!(literals);
//         })
//     });
// }

// #[test]
// fn nowdoc_string_literal() {
//     glob!("fixtures/nowdoc_string_literals.php", |path| {
//         let file = fs::read_to_string(path).unwrap();
//         let pairs = PHPParser::parse(PHPRule::SCRIPT, &file)
//             .expect("Something went wrong parsing the file");
//         let literals =
//             Extractor::default().extract_nodes(pairs, PHPRule::STRING_LITERAL);

//         with_settings!({prepend_module_to_snapshot => false}, {
//             insta::assert_yaml_snapshot!(literals);
//         })
//     });
// }
