// Standard Uses
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// Crate Uses

// External Uses
use eyre::{anyhow, bail, Context, ContextCompat, Result};
use glob::glob;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Group {
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub groups: HashMap<String, Group>,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub groups: HashMap<String, Group>,
}

impl Project {
    pub fn from_path(path: &Path) -> Result<Self> {
        let search_path = if path.is_relative() {
            path.to_path_buf()
        } else {
            PathBuf::from(format!(
                "{}/{}",
                std::env::current_dir()?.to_str().unwrap(),
                path.to_str().unwrap()
            ))
        };

        let mut search = glob(format!("{}project.*", search_path.to_str().unwrap()).as_str())?;

        let project = search.next().context(format!(
            "Could not find any file named 'project.<extension>' at '{}'",
            path.to_str().unwrap()
        ))??;

        let extension = project.extension().unwrap().to_str().unwrap();
        let content = fs::read_to_string(&project).context("Could not read file")?;

        from_extension(content.as_str(), extension)
    }
}

fn from_extension(content: &str, extension: &str) -> Result<Project> {
    match extension {
        "json5" => from_json5(content),
        &_ => {
            bail!("Cannot parse given format extension {}", extension)
        }
    }
}

pub fn from_json5(content: &str) -> Result<Project> {
    json5::from_str(content).map_err(|err| anyhow!({ err }))
}
