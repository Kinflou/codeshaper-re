// Relative Modules
pub mod parser;

// Standard Uses

// Crate Uses
use crate::external::nav_ast_pest::small_config::parser::parse_raw;
use crate::navigation_ast::Navigator;

// External Uses


pub struct SmallConfigNavigator;
impl SmallConfigNavigator {
    pub fn new_box() -> Box<dyn Navigator> { Box::new(Self {})}

}

impl Navigator for SmallConfigNavigator {
    fn navigate(&self, expression: &str) {
        parse_raw(expression);
    }
}