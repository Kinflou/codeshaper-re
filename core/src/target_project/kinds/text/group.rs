// Standard Uses
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::{Rc, Weak};

// Crate Uses
use crate::target_project::{File, Group, Target};

// External Uses


pub struct TextGroup {
    pub name: String,
    pub root: Option<Rc<RefCell<dyn Target>>>,
    pub parent: Option<Weak<RefCell<dyn Group>>>,
    pub groups: Vec<Weak<RefCell<dyn Group>>>,
    pub files: Vec<Weak<RefCell<dyn File>>>
}

impl TextGroup {
    pub fn new_shared(name: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self {
                name,
                root: None,
                parent: None,
                groups: vec![],
                files: vec![]
            }
        ))
    }
}

impl Debug for TextGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextGroup")
            .field("groups", &self.groups)
            .finish()
    }
}

impl Group for TextGroup {
    fn name(&self) -> &String { &self.name }
    fn root(&self) -> &Option<Rc<RefCell<dyn Target>>> { &self.root }
    fn parent(&self) -> &Option<Weak<RefCell<dyn Group>>> { &self.parent }
    fn groups(&self) -> &Vec<Weak<RefCell<dyn Group>>> { &self.groups }
    fn files(&self) -> &Vec<Weak<RefCell<dyn File>>> { &self.files }

    fn add_root(&mut self, root: Rc<RefCell<dyn Target>>) {
        self.root = Some(root);
    }

    fn add_file(&mut self, file: Weak<RefCell<dyn File>>) {
        self.files.push(file);
    }
}
