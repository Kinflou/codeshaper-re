// Standard Uses
use std::fs;
use std::path::Path;

// Crate Uses

// External Uses
use eyre::{anyhow, bail, Context, ContextCompat, Result};
use glob::glob;
use knuffel::parse;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, knuffel::Decode)]
pub struct ProjectConfig {
    pub name: String,
    pub target: String,
    pub ast_set: String,
    pub description: String,
}

impl ProjectConfig {
    pub fn from_path(path: &Path) -> Result<Self> {
        let pattern = format!("{}/config.*", path.to_str().unwrap());
        let mut search = glob(pattern.as_str())
            .context("Failed to search for glob pattern")
            .unwrap();

        let settings = search
            .next()
            .context(format!(
                "Could not find any file named 'config.*' as '*' being a\
                    \nsupported extension at {}",
                path.to_str().unwrap()
            ))
            .unwrap()?;

        let extension = settings.extension().unwrap().to_str().unwrap();
        let content = fs::read_to_string(&settings)
            .context("Could not read file")
            .unwrap();

        from_extension(content.as_str(), extension)
    }
}

fn from_extension(content: &str, extension: &str) -> Result<ProjectConfig> {
    match extension {
        "json5" => from_json5(content),
        "kdl" => from_kdl(content),
        &_ => {
            bail!(
                "Project settings extension '{}' is not supported",
                extension
            )
        }
    }
}

// TODO: Likely we should only have one format, and that would be KDL, so see what's necessary
//       to remove this
fn from_json5(content: &str) -> Result<ProjectConfig> {
    json5::from_str(content).map_err(|err| anyhow!("{}", err))
}

fn from_kdl(content: &str) -> Result<ProjectConfig> {
    parse::<ProjectConfig>("settings.kdl", content).map_err(|err| anyhow!("{:#?}", err))
}
