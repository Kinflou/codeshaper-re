// Standard Uses
use std::path::Path;

// Crate Uses
use crate::workspace::config::WorkspaceConfig;
use crate::project::Project;

// External Uses
use eyre::{Result};
use stopwatch::Stopwatch;


pub struct Operation {
    pub configuration: WorkspaceConfig,
    pub stop_watch: Stopwatch,
    pub project: Option<Project>,
}


impl Operation {
    pub fn from_config(config: WorkspaceConfig) -> Result<Self> {
        let path = Path::new(config.project.as_str());

        let project = Project::from_directory(path)?;

        Ok(Self {
            configuration: config,
            stop_watch: Default::default(),
            project: Some(project),
        })
    }

    pub fn start(&mut self) {
        // self.project.unwrap().file_map.target_file.controller;
    }

    pub fn stop(&self) {
        todo!()
    }

    pub fn next(&self) {
        todo!()
    }
}

