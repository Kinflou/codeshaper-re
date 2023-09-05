// Standard Uses
use std::marker::PhantomData;

// Crate Uses
use crate::target_project::{Target};

// External Uses
use indextree::NodeId;


#[allow(unused)]
#[derive(Debug)]
pub struct FileIter<'a, T> where T: Target {
    pub(crate) target: &'a mut Box<T>,
    current: NodeId,
    file_index: usize,
    p: PhantomData<&'a FileIter<'a, T>>
}
#[allow(unused)]
impl<'a, T> FileIter<'a, T> where T: Target {
    pub(crate) fn iter(target: &'a mut Box<T>, root_id: NodeId) -> Self {
        Self { target, current: root_id, file_index: 0, p: Default::default() }
    }
}

/*
impl<'a, T> Iterator for FileIter<'a, T> where T: Target {
    type Item = &'a <<T as Target>::Group as Group>::File;

    fn next(&mut self) -> Option<Self::Item> {
        /*
        let current_node = self.target.graph().get(self.current).unwrap();
        let current_group = current_node.get();

        if current_group.files().len() == self.file_index {
            if let Some(next) = current_node.next_sibling() {
                self.current = next;
                self.file_index = 0;

                return self.next()
            }

            return None
        }

        return Some(&current_group.files()[self.file_index])
        */
    }
}
*/
