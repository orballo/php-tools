use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser {}

pub fn main() {
    let successful_parse = CSVParser::parse(Rule::field, "-273.15");
    eprintln!("successful_parse = {:#?}", successful_parse);

    let unsuccessful_parse = CSVParser::parse(Rule::field, "this is not a number");
    eprintln!("unsuccessful_parse = {:#?}", unsuccessful_parse);
}
