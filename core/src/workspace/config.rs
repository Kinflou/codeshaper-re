// Standard Uses
use std::fs;
use std::fs::Metadata;
use std::path::Path;

// Crate Uses

// External Uses
use eyre::{bail, Context, Result};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub name: String,
    pub project: String,
    pub target: String,
    pub output: String,

    #[serde(default)]
    pub backup: String,

    pub result: ResultOptions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResultOptions {
    Replace,
    BackupAndReplace,
    CreateNew,
}

impl WorkspaceConfig {
    pub fn from_file(path: &Path) -> Result<Self> {
        let extension = path.extension().unwrap().to_str().unwrap();
        let content = fs::read_to_string(path)
            .context("Could not load operation configuration")
            .unwrap();

        from_extension(content.as_str(), extension)
    }

    pub fn get_real_project_directory(&self) -> Result<Metadata> {
        /*
        let directory = format!("{}/{}", &self.project, &self.project);

        if fs::metadata(&directory).unwrap().is_dir() {
            return Ok(fs::metadata(&directory).unwrap());
        }
        */

        if fs::metadata(&self.project).unwrap().is_dir() {
            return Ok(fs::metadata(&self.project).unwrap());
        }

        bail!("Could not determine directory")
    }
}

fn from_extension(content: &str, extension: &str) -> Result<WorkspaceConfig> {
    match extension {
        "json5" => from_json5(content),
        "kdl" => from_kdl(content),
        &_ => {
            bail!("")
        }
    }
}

pub fn from_json5(content: &str) -> Result<WorkspaceConfig> {
    json5::from_str(content).context("Could not load JSON5 operation settings")
}

#[allow(unused)]
pub fn from_kdl(content: &str) -> Result<WorkspaceConfig> {
    todo!()
}
