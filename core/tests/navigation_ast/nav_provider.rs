// Standard USes

// Crate Uses

// External Uses
use codeshaper_core::navigation_ast;


#[test]
fn find_provider() {
    let provider = navigation_ast::find_provider("pest");

    provider.unwrap();
}

#[test]
fn find_provider_and_navigator() {
    let provider = navigation_ast::find_provider("pest").unwrap();
    let navigator = provider.find_navigator("small_config");

    navigator.unwrap();
}

#[test]
fn find_provider_and_navigator_by_namespace() {
    let namespace = "pest.small_config";
    let navigator = navigation_ast::find_navigator_by_namespace(namespace);

    navigator.unwrap();
}
