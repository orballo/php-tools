use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/base.pest"]
#[grammar = "grammar/keywords.pest"]
#[grammar = "grammar/operators.pest"]
#[grammar = "grammar/punctuators.pest"]
#[grammar = "grammar/names.pest"]
#[grammar = "grammar/literals.pest"]
pub struct PHPParser {}
