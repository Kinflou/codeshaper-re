// Standard Uses
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::rc::{Rc, Weak};

// Crate Uses
use crate::target_project::iterator::TargetIter;
use crate::target_project::{Group, Target, TargetAlias};
use crate::workspace::operation::Operation;

// External Uses
use eyre::Result;

pub struct VCXSolution<'a> {
    file_iter: Option<TargetIter<'a>>,
}

#[allow(unused)]
impl VCXSolution<'_> {
    pub fn from_path_shared(target: &Path) -> Result<Rc<RefCell<dyn Target>>> {
        todo!()
    }
}

#[allow(unused)]
impl Debug for VCXSolution<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[allow(unused)]
impl<'a> Target for VCXSolution<'a> {
    fn name(&self) -> &String {
        todo!()
    }

    fn groups(&self) -> &Vec<Weak<RefCell<dyn Group>>> {
        todo!()
    }

    fn operation(&self) -> &Option<Operation> {
        todo!()
    }

    fn ast_set(&self) {
        todo!()
    }

    /*
    fn file_map(&mut self) -> &mut Option<TargetIter<'_>> {
        &mut self.file_iter
    }
    */

    fn add_group(&mut self, group: Weak<RefCell<dyn Group>>) {
        todo!()
    }
}

impl TargetAlias for VCXSolution<'_> {
    const ALIAS: &'static str = "vcx";
}
