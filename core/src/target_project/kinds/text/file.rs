// Standard Uses
use std::cell::RefCell;
use std::rc::Rc;

// Crate Uses
use crate::target_project::{File, FileState, Group};

// External Uses

#[derive(Debug, Default)]
pub struct TextFile {
    pub name: String,
    pub content: String,

    pub parent: Option<Box<dyn Group>>,
    // pub controller: Option<Controller>,
    pub state: FileState
}

impl TextFile {
    pub fn new_shared(name: String, content: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self {
                name,
                content,
                parent: None,
                // controller: None,
                state: Default::default(),
            }
        ))
    }
}

impl File for TextFile {
    fn name(&self) -> &String { &self.name }
    fn parent(&self) -> &Option<Box<dyn Group>> { &self.parent }
    // fn controller(&self) -> &Option<Controller> { &self.controller }
    fn state(&self) -> &FileState { &self.state }
}

