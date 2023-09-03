// Standard Uses
use std::path::Path;

// Crate Uses

// External Uses
use codeshaper_core::shaping::patch::{Actions, Patch};
use codeshaper_core::shaping::patch::builder::Builder;
use codeshaper_core::shaping::patch::maker::Maker;
use codeshaper_core::shaping::patch::replacer::Replacer;


const PATCH_PATH: &str = "tests/data/patches/hello.kdl";

#[test]
pub fn load_patch_kdl() {
    let path = Path::new(PATCH_PATH);
    let patch = Patch::from_path(path).unwrap();

    pretty_assertions::assert_eq!(patch,
        Patch {
            enabled: true,
            alias: None,
            file: "greet.scl".to_owned(),
            actions: Actions {
                builders: vec![
                    Builder {
                        name: "hello_library".to_owned(),
                        location: "field".to_owned(),
                        reference_location: None,
                        r#match: "Lets greet (.*?)?".to_owned(),
                        build: "Hello #[library_sentence](\\g<1>)!".to_owned(),
                        actions: Default::default()
                    },
                ],
                makers: vec![
                    Maker {
                        name: "library_sentence".to_string(),
                        arguments: vec!["name".to_owned()],
                        prepare: None,
                        make: "$[name] Library".to_string(),
                    }
                ],
                replacers: vec![
                    Replacer {
                        name: "greet_library".to_owned(),
                        location: "field".to_owned(),
                        from: "".to_string(),
                        to: "#[hello_library]".to_string(),
                        // flags: None,
                        reference_location: None,
                        reference: None,
                        actions: Default::default(),
                    }
                ],
                resolvers: vec![],
            }
        }
    );
}

