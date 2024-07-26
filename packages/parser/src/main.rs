use crate::lexer::Token;
use crate::parser::*;
use logos::Logos;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::error::{Error, ErrorKind};
use nom::IResult;

fn main() {
    let source = "<?php echo 'Hello, World!';";
    let lexer: Vec<Token> = Token::lexer(source)
        .filter_map(|token| token.ok())
        .collect();
    let result = parse_string(&lexer);
    println!("{:#?}", result);
}
