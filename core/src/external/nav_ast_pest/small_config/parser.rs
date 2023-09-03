// Standard Uses

// Crate Uses

// External Uses
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;



#[derive(Parser)]
#[grammar = "src/external/nav_ast_pest/small_config/grammar.pest"]
pub struct SmallConfigParser;

#[allow(unused)]
pub fn from_path(path: &str) {
    let raw = std::fs::read_to_string(path).unwrap();

    parse_raw(raw.as_str());
}

pub fn parse_raw(content: &str) {
    let pairs = SmallConfigParser::parse(Rule::config, content).unwrap();
    let mut unit = vec![];

    for pair in pairs { unit.push(parse_inner(pair)) }
}


pub fn parse_inner(pair: Pair<Rule>) {
    match pair.as_rule() {
        Rule::field => {

        },
        Rule::nested => {

        }
        missing => unimplemented!("Rule not implemented: {:?}", missing)
    }
}

