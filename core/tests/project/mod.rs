// Standard Uses
use std::path::Path;

// Crate Uses
use crate::TEST_WORKSPACE_DIR;

// External Uses
use once_cell::sync::Lazy;
use codeshaper_core::project::settings::ProjectConfig;


static TEST_PROJECT_PATH: Lazy<String> = Lazy::new(||
    format!("{}/shaping_project/", TEST_WORKSPACE_DIR)
);

#[test]
fn load_project_config() {
    let config = ProjectConfig::from_path(
        Path::new(&*TEST_PROJECT_PATH)
    ).unwrap();

    pretty_assertions::assert_eq!(
        config,
        ProjectConfig {
            name: "Test Shaping Project".to_string(),
            target: "text".to_string(),
            ast_set: "pest.small_config".to_string(),
            description: "A little test with some cases".to_string(),
        }
    );
}
