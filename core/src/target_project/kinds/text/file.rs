// Standard Uses

// Crate Uses
use crate::target_project::{File, FileState};

// External Uses
use indextree::NodeId;


#[derive(Debug, PartialEq)]
pub struct TextFile {
    pub name: String,
    pub content: String,

    pub parent: NodeId,
    // pub controller: Option<Controller>,
    pub state: FileState,
}

impl File for TextFile {
    fn name(&self) -> &String {
        &self.name
    }
    fn parent(&self) -> &NodeId {
        &self.parent
    }
    // fn controller(&self) -> Option<&Controller> { &self.controller }
    fn state(&self) -> &FileState {
        &self.state
    }
}
