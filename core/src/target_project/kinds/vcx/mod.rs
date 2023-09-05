// Standard Uses
use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::PathBuf;

// Crate Uses
use crate::workspace::operation::Operation;
use crate::target_project::file_iter::FileIter;
use crate::target_project::{Group, Target, TargetAlias};
use crate::target_project::kinds::text::file::TextFile;

// External Uses
use eyre::Result;
use indextree::{Arena, NodeId};


#[allow(unused)]
#[derive(Debug)]
pub struct VCXSolution<'a> {
    name: String,
    graph: Arena<VCXGroup<'a>>,
    root_group: Option<NodeId>,
    file_iter: Option<FileIter<'a, VCXSolution<'a>>>
}
#[allow(unused)]
impl<'a> VCXSolution<'a> {
    pub fn from_path(target: PathBuf) -> Result<Box<Self>> {
        todo!()
    }
}

impl<'a> Target for VCXSolution<'a> {
    type Group = VCXGroup<'a>;

    fn name(&self) -> &String {
        &self.name
    }

    fn graph(&mut self) -> &mut Arena<Self::Group> { &mut self.graph }

    fn root_group(&self) -> &Option<NodeId> { &self.root_group }

    fn operation(&self) -> Option<&Operation> {
        todo!()
    }

    /*
    fn file_map(&mut self) -> &mut Option<TargetIter<'_>> {
        &mut self.file_iter
    }
    */

    fn ast_set(&self) {
        todo!()
    }
}


impl<'a> TargetAlias for VCXSolution<'a> {
    const ALIAS: &'static str = "vcx";
}

#[derive(Debug)]
pub struct VCXGroup<'a> {
    name: String,
    parent: Option<NodeId>,
    groups: Vec<NodeId>,
    _phantom: &'a PhantomData<Self>,
}

#[allow(unused)]
impl<'a> Group for VCXGroup<'a> {
    type Group = VCXGroup<'a>;
    type File = TextFile;

    fn name(&self) -> &String { &self.name }

    fn parent(&self) -> &Option<NodeId> { &self.parent }

    fn groups(&self) -> &Vec<NodeId> { &self.groups }

    fn files(&self) -> &Vec<Self::File> { todo!() }

    fn add_group(&mut self, group: Self::Group) -> NodeId { todo!() }
}