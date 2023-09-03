// Standard Uses

// Crate Uses
use crate::expression::syntax::Expression;

// External Uses
use eyre::Result;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;


#[allow(unused)]
#[derive(Parser)]
#[grammar = "src/expression/syntax/expression.pest"]
pub struct ActionParser;



pub fn parse_expression(input: &str) -> Result<Expression> {
    let expression = ActionParser::parse(Rule::action, input)?.next().unwrap();

    fn parse_expression(pair: Pair<Rule>) -> Expression {
        match pair.as_rule() {
            Rule::action => Expression::Action(
                pair.into_inner().map(|pair| {
                    let item = parse_expression(pair);

                    item
                }).collect::<Vec<Expression>>(),
            ),
            Rule::expression => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();

                let arguments = inner.next();
                if arguments.is_none() {
                    return Expression::Expr { name, args: vec![] }
                }

                let args = arguments.unwrap().into_inner()
                    .map(|a| parse_expression(a)).collect();

                Expression::Expr { name, args }
            },
            Rule::inner_text => Expression::Text(pair.as_str()),
            missing => unreachable!("Rule not implemented {:?}", missing)
        }
    }

    Ok(parse_expression(expression))
}


