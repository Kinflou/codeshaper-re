// Relative Modules
pub mod file;
pub mod group;
pub mod project;

// Standard Uses
use std::fmt::Debug;
use std::path::PathBuf;

// Crate Uses
use crate::target_project::kinds::text::group::TextGroup;
use crate::target_project::kinds::text::project::Project;
use crate::target_project::{Target, TargetAlias};
use crate::workspace::operation::Operation;

// External Uses
use eyre::Result;
use indextree::{Arena, NodeId};


#[derive(Debug, PartialEq)]
pub struct TextSolution<'a> {
    pub name: String,
    pub graph: Arena<TextGroup<'a>>,
    pub root_group: Option<NodeId>,
    // pub file_iter: Option<FileIter<'a, TextGroup<'a>>>,
}

#[allow(unused)]
impl<'a> TextSolution<'a> {
    pub fn new(name: String, root_group: TextGroup) -> Self {
        let mut graph = Arena::<TextGroup>::new();
        let root_id = graph.new_node(root_group);

        Self {
            name, graph: Default::default(), root_group: Some(root_id),
        }
    }

    pub fn from_target(path: PathBuf) -> Result<Box<TextSolution<'a>>> {
        let project = Project::from_path(path)?;
        let graph = Arena::<TextGroup>::new();

        let target = Self {
            name: "TODO!".to_string(), graph,
            root_group: None,
            // file_iter: None,
        };

        let mut target = Box::new(target);

        let root = TextGroup::new(
            &mut target, None, "root".to_owned(),
            vec![], vec![]
        );

        let root_id = target.graph.new_node(root);
        // target_mut.file_iter = Some(FileIter::<TextGroup>::iter(target.clone(), root_id));
        let target = target;

        Ok(target)
    }
}

#[allow(unused)]
impl<'a> Target for TextSolution<'a> {
    type Group = TextGroup<'a>;

    fn name(&self) -> &String {
        &self.name
    }

    fn graph(&mut self) -> &mut Arena<Self::Group> { &mut self.graph }

    fn root_group(&self) -> &Option<NodeId> { &self.root_group }

    fn operation(&self) -> Option<&Operation> {
        todo!()
    }
    fn ast_set(&self) {
        todo!()
    }
}

impl<'a> TargetAlias for TextSolution<'a> {
    const ALIAS: &'static str = "txt";
}
