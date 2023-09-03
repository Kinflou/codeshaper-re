// Standard Uses

// Crate Uses
use crate::expression::syntax::Expression;

// External Uses

#[allow(unused)]
pub trait ExpressionVisitor {
    fn navigate(&mut self, expr: Expression)
    where
        Self: Sized,
    {
        parse_expression(self, expr)
    }
    fn visit_expression(&mut self, name: &str, args: Vec<Expression>);
    fn visit_text(&mut self, text: &str);
}

pub fn navigate_expression<'a>(visitor: &'a mut dyn ExpressionVisitor, nodes: Expression) {
    parse_expression(visitor, nodes)
}

fn parse_expression<'a>(visitor: &'a mut dyn ExpressionVisitor, expr: Expression<'a>) {
    use Expression::*;
    match expr {
        Action(expressions) => {
            expressions
                .into_iter()
                .for_each(|x| parse_expression(visitor, x));
        }
        Expr { name, args } => {
            visitor.visit_expression(name, args);
        }
        Text(text) => {
            visitor.visit_text(text);
        }
    }
}
