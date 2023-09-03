use std::any::Any;
// Standard Uses
use downcast_rs::{impl_downcast, Downcast};
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

// Crate Uses

// External Uses
use indextree::{Arena, NodeId};

pub trait Target: Debug {
    fn root_group(&self) -> &Rc<RefCell<dyn Group>>;

    fn map(&self) -> &Option<TargetIter>;

    fn iter(&self) -> TargetIter {
        let mut arena: Arena<Rc<RefCell<dyn Group>>> = Arena::new();

        fn recurse_new(
            arena: &mut Arena<Rc<RefCell<dyn Group>>>,
            node_id: NodeId,
            group: &Rc<RefCell<dyn Group>>,
        ) {
            let id = arena.new_node(group.clone());
            let group_ref = group.borrow();

            for sub_group in group_ref.groups().iter() {
                recurse_new(arena, id, sub_group);
            }

            node_id.append(id, arena);
        }

        let root_id = arena.new_node(Rc::clone(self.root_group()));
        let root_group = self.root_group().borrow();

        for group in root_group.groups() {
            recurse_new(&mut arena, root_id, group);
        }

        TargetIter {
            arena,
            current: root_id,
            file_index: 0,
        }
    }

    fn refresh_state(&mut self);
}

pub trait Group: Debug + Downcast {
    fn groups(&self) -> &Vec<Rc<RefCell<dyn Group>>>;
    fn add_group(&mut self, group: Rc<RefCell<dyn Group>>);
    fn files(&self) -> &Vec<Rc<RefCell<dyn File>>>;
}
impl_downcast!(Group);

pub trait File: Debug {
    fn name(&self) -> &String;
}

#[derive(Debug)]
pub struct TargetIter {
    pub(crate) arena: Arena<Rc<RefCell<dyn Group>>>,
    pub(crate) current: NodeId,
    pub(crate) file_index: usize,
}

impl Iterator for TargetIter {
    type Item = Rc<RefCell<dyn File>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.arena.get(self.current).cloned().unwrap();

        let group_shared = node.get();
        let group = group_shared.borrow();

        if group.files().len() == self.file_index {
            if let Some(next) = node.next_sibling() {
                self.current = next;
                self.file_index = 0;

                return self.next();
            };

            if let Some(child) = node.first_child() {
                self.current = child;
                self.file_index = 0;

                return self.next();
            }

            if let Some(parent) = node.parent() {
                self.current = parent;
                self.file_index = 0;

                return self.next();
            };

            return None;
        }

        let file = &group.files()[self.file_index];
        self.file_index += 1;

        return Some(file.clone());
    }
}
