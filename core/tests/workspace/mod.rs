// Standard Uses
use std::path::Path;
use once_cell::sync::Lazy;

// Crate Uses
use crate::TEST_WORKSPACE_DIR;

// External Uses
use codeshaper_core::workspace::config::{ResultOptions, WorkspaceConfig};
use codeshaper_core::workspace::operation::Operation;


static TEST_WORKSPACE_CONFIG_PATH: Lazy<String> = Lazy::new(||
    format!("{}/config.json5", TEST_WORKSPACE_DIR)
);

#[test]
fn load_workspace_config() {
    let config = WorkspaceConfig::from_file(
        Path::new(&*TEST_WORKSPACE_CONFIG_PATH)
    ).unwrap();

    pretty_assertions::assert_eq!(config,
        WorkspaceConfig {
            name: "Test Workspace".to_string(),
            project: "tests/data/test_workspace/shaping_project/".to_string(),
            target: "tests/data/test_workspace/target/origin/".to_string(),
            output: "tests/data/test_workspace/target/expected_result/".to_string(),
            backup: "tests/data/test_workspace/target/backup/".to_string(),
            result: ResultOptions::BackupAndReplace,
        }
    );
}

#[test]
fn load_workspace_operation() {
    let config = WorkspaceConfig::from_file(
        Path::new(&*TEST_WORKSPACE_CONFIG_PATH)
    ).unwrap();

    let workspace = Operation::from_config(config.clone());

    workspace.unwrap();
}
