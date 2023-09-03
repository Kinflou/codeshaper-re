// Relative Modules
pub mod parser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expression<'a> {
    Action(Vec<Expression<'a>>),
    Expr {
        name: &'a str,
        args: Vec<Expression<'a>>
    },
    Text(&'a str),
}
