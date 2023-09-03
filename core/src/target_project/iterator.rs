use std::cell::RefCell;
// Standard Uses
use std::rc::Weak;

// Crate Uses
use crate::target_project::Group;

// External Uses



pub(crate) struct TargetIter<'a> {
    pub(crate) parent: Option<Box<TargetIter<'a>>>,
    pub(crate) children: &'a [Weak<RefCell<dyn Group>>],
    pub(crate) previous: Option<&'a Weak<RefCell<dyn Group>>>,
    pub(crate) file_index: usize
}
