use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/base.pest"]
#[grammar = "grammar/lexical/keywords.pest"]
#[grammar = "grammar/lexical/operators.pest"]
#[grammar = "grammar/lexical/punctuators.pest"]
#[grammar = "grammar/lexical/names.pest"]
#[grammar = "grammar/lexical/literals.pest"]
pub struct PHPParser {}
