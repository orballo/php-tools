use insta::{assert_yaml_snapshot, glob, with_settings};
use parser::{PHPParser, PHPRule};
use pest::iterators::Pairs;
use pest::Parser;
use serde::Serialize;
use std::fs;

#[derive(Debug, Clone, Serialize)]
struct Comment {
    content: String,
    starting_line: usize,
    ending_line: usize,
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            content: String::new(),
            starting_line: 0,
            ending_line: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct Node {
    name: String,
    content: String,
    line: usize,
    length: usize,
}

#[derive(Debug)]
struct Extractor {
    comment: Comment,
    nodes: Vec<Node>,
}

impl Default for Extractor {
    fn default() -> Self {
        Self {
            comment: Comment::default(),
            nodes: Vec::new(),
        }
    }
}

impl Extractor {
    fn extract_nodes(
        &mut self,
        pairs: Pairs<PHPRule>,
        rule: PHPRule,
    ) -> Vec<Node> {
        for pair in pairs {
            match pair.as_rule() {
                // Parse comment to extract test name.
                PHPRule::SINGLE_LINE_COMMENT => {
                    self.comment.content = pair
                        .as_str()
                        .trim_matches(|c| c == '/' || c == ' ')
                        .to_string();
                    self.comment.starting_line =
                        pair.as_span().start_pos().line_col().0;
                    self.comment.ending_line =
                        pair.as_span().end_pos().line_col().0;
                }
                // Extract the node information and add test name
                // if there is a comment one line above.
                _ if pair.as_rule() == rule => {
                    let content = pair.as_str().to_string();
                    let line = pair.as_span().start_pos().line_col().0;
                    let length = pair.as_span().end() - pair.as_span().start();

                    let name = if self.comment.ending_line + 1 == line {
                        self.comment.content.clone()
                    } else {
                        String::new()
                    };

                    self.nodes.push(Node {
                        name,
                        content,
                        line,
                        length,
                    });
                }
                // For any other rule we recursively call this function.
                _ => {
                    self.extract_nodes(pair.into_inner(), rule);
                }
            }
        }

        self.nodes.clone()
    }
}

#[test]
fn single_quoted_string_literal() {
    glob!("fixtures/single_quoted_string_literals.php", |path| {
        let file = fs::read_to_string(path).unwrap();
        let pairs = PHPParser::parse(PHPRule::SCRIPT, &file)
            .expect("Something went wrong parsing the file");
        let literals = Extractor::default()
            .extract_nodes(pairs, PHPRule::SINGLE_QUOTED_STRING_LITERAL);

        with_settings!({prepend_module_to_snapshot => false}, {
            assert_yaml_snapshot!(literals);
        })
    });
}
