// Relative Modules
pub mod small_config;

// Standard Uses
use once_cell::sync::Lazy;
use std::collections::HashMap;

// Crate Uses
use crate::external::nav_ast_pest::small_config::SmallConfigNavigator;
use crate::navigation_ast::{Navigator, NavigatorProvider};

// External Uses

pub static NAVIGATORS: Lazy<HashMap<&str, fn() -> Box<dyn Navigator>>> =
    Lazy::new(|| HashMap::from([("small_config", SmallConfigNavigator::new_box as _)]));

pub struct PestNavProvider;
#[allow(unused)]
impl PestNavProvider {
    pub fn new() -> Self {
        Self {}
    }
    pub fn new_box() -> Box<dyn NavigatorProvider> {
        Box::new(Self {})
    }
}

#[allow(unused)]
impl NavigatorProvider for PestNavProvider {
    fn find_navigator(&self, name: &str) -> Option<fn() -> Box<dyn Navigator>> {
        if let Some((key, val)) = NAVIGATORS.get_key_value(name) {
            return Some(val.clone());
        }

        None
    }
}
