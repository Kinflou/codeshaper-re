// Relative Modules
pub(crate) mod file_iter;
pub mod kinds;

// Standard Uses
use std::any::Any;
use std::fmt::Debug;
use std::path::PathBuf;
use downcast_rs::Downcast;

// Crate Uses
use crate::workspace::operation::Operation;

// External Uses
use indextree::{Arena, NodeId};
use eyre::{Result, bail};


pub trait File: Debug {
    fn name(&self) -> &String;
    fn parent(&self) -> &NodeId;
    // fn controller(&self) -> Option<&Controller>;
    fn state(&self) -> &FileState;
}

pub trait Target: Debug {
    type Group: Group;

    fn name(&self) -> &String;
    fn graph(&mut self) -> &mut Arena<Self::Group>;
    fn root_group(&self) -> &Option<NodeId>;

    fn operation(&self) -> Option<&Operation>;
    fn ast_set(&self);
}

pub trait TargetAlias: Target {
    const ALIAS: &'static str;
}

pub trait Group: Debug {
    type Group: Group;
    type File: File;

    fn name(&self) -> &String;
    fn parent(&self) -> &Option<NodeId>;
    fn groups(&self) -> &Vec<NodeId>;
    fn files(&self) -> &Vec<Self::File>;

    fn add_group(&mut self, group: Self::Group) -> NodeId;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FileState {
    #[default]
    Untouched,
    Waiting,
    Processing,
    Processed,
    Error,
}

#[allow(unused)]
pub fn from_kind_path(kind: &str, path: PathBuf) -> Result<Box<dyn Any>> {
    use crate::target_project::kinds::text::TextSolution;
    use crate::target_project::kinds::vcx::VCXSolution;

    return match kind {
        "plain_text" => Ok(TextSolution::from_target(path)?),
        "visual_studio.vcx" => Ok(VCXSolution::from_path(path)?.into_any()),
        _ => bail!("The target kind '{}' is not supported", kind),
    };
}
