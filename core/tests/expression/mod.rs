// Standard Uses

// Crate Uses

// External Uses
use codeshaper_core::expression::syntax::{parser, Expression};
use codeshaper_core::expression::visitor::{navigate_expression, ExpressionVisitor};

pub struct TestVisitor {
    parts: Vec<String>,
}

#[allow(unused)]
impl ExpressionVisitor for TestVisitor {
    fn visit_expression(&mut self, name: &str, args: Vec<Expression>) {
        // The reason we just directly get the argument text here is just to
        // pretend we resolved the given action and its arguments
        let Expression::Text(arg) = args[0] else {
            panic!()
        };

        self.parts.push(arg.to_owned());
    }

    fn visit_text(&mut self, text: &str) {
        self.parts.push(text.to_owned());
    }
}

#[test]
pub fn visit_expression_endpoints() {
    let expression = "\"A\" #[greet](\"hello\") \"world\"";
    let ast = parser::parse_expression(expression).unwrap();
    let mut visitor = TestVisitor { parts: vec![] };

    navigate_expression(&mut visitor, ast);

    pretty_assertions::assert_eq!(visitor.parts, vec!["A", "hello", "world"]);
}
