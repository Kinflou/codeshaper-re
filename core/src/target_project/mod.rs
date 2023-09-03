// Relative Modules
pub mod file_map;
pub(crate) mod iterator;
pub mod kinds;

// Standard Uses
use std::cell::RefCell;
use std::fmt::Debug;
use std::path::Path;
use std::rc::{Rc, Weak};

// Crate Uses
use crate::target_project::iterator::TargetIter;
use crate::target_project::kinds::KINDS_PROVIDER;
use crate::workspace::operation::Operation;

// External Uses
use eyre::{bail, Result};

pub trait File: Debug {
    fn name(&self) -> &String;
    fn parent(&self) -> &Option<Box<dyn Group>>;
    // fn controller(&self) -> &Option<Controller>;
    fn state(&self) -> &FileState;
}

pub trait Target: Debug {
    fn name(&self) -> &String;
    fn groups(&self) -> &Vec<Weak<RefCell<dyn Group>>>;
    fn operation(&self) -> &Option<Operation>;
    fn ast_set(&self);
    fn add_group(&mut self, group: Weak<RefCell<dyn Group>>);

    fn file_map(&mut self) -> &mut Option<TargetIter<'_>> {
        todo!()
    }
    fn iter(&self) -> TargetIter {
        TargetIter {
            children: self.groups().as_slice(),
            previous: None,
            parent: None,
            file_index: 0,
        }
    }
}

pub trait TargetAlias: Target {
    const ALIAS: &'static str;
}

pub trait Group: Debug {
    fn name(&self) -> &String;
    fn root(&self) -> &Option<Rc<RefCell<dyn Target>>>;
    fn parent(&self) -> &Option<Weak<RefCell<dyn Group>>>;
    fn groups(&self) -> &Vec<Weak<RefCell<dyn Group>>>;
    fn files(&self) -> &Vec<Weak<RefCell<dyn File>>>;

    fn add_root(&mut self, root: Rc<RefCell<dyn Target>>);
    fn add_file(&mut self, file: Weak<RefCell<dyn File>>);
}

#[allow(unused)]
pub fn from_kind_path(kind: &str, path: &'_ Path) -> Result<Rc<RefCell<dyn Target>>> {
    let Some((_, init_fn)) = KINDS_PROVIDER.get_key_value(kind) else {
        bail!("The target kind '{}' is not supported", kind)
    };

    init_fn(path)
}

pub fn find_initializer(kind: &str) -> Option<&fn(&Path) -> Result<Rc<RefCell<dyn Target>>>> {
    let Some((_, init_fn)) = KINDS_PROVIDER.get_key_value(kind) else {
        return None;
    };

    Some(init_fn)
}

#[derive(Debug, Clone, Default)]
pub enum FileState {
    #[default]
    Untouched,
    Waiting,
    Processing,
    Processed,
    Error,
}
