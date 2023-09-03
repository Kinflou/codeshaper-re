// Standard Uses
use std::collections::HashMap;

// Crate Uses
use crate::external::nav_ast_pest::PestNavProvider;

// External Uses
use once_cell::sync::Lazy;


pub trait NavigatorProvider {
    fn find_navigator(&self, name: &str) -> Option<fn() -> Box<dyn Navigator>>;
}

pub trait Navigator {
    fn navigate(&self, expression: &str);
}


#[allow(unused)]
pub static NAVIGATOR_PROVIDERS: Lazy<HashMap<&str, fn() -> Box<dyn NavigatorProvider>>> = Lazy::new(||
    HashMap::from([
        ("pest", PestNavProvider::new_box as _)
    ])
);


pub fn find_provider(name: &str) -> Option<Box<dyn NavigatorProvider>> {
    let provider = NAVIGATOR_PROVIDERS.get_key_value(name);

    if provider.is_none() { return None }

    Some(provider.unwrap().1())
}

pub fn find_navigator_by_namespace(namespace: &str) -> Result<Box<dyn Navigator>, String> {
    let parts = namespace.split('.').collect::<Vec<&str>>();

    if parts.len() != 2 {
        return Err(
            format!(
                "Namespace must have only 2 parts, but it has {}.\n\
                It has to be like <provider>.<navigator>, example: pest.small_config",
                parts.len()
            ).to_owned()
        )
    }

    let provider_name = parts[0]; let navigator_name = parts[1];

    let Some(provider) = find_provider(provider_name) else {
        return Err(format!("No provider found by the name '{}'", provider_name))
    };

    let Some(navigator) = provider.find_navigator(navigator_name) else {
        return Err(
            format!("No navigator named '{}' found in provider '{}'", navigator_name, provider_name)
        )
    };

    Ok(navigator())
}

