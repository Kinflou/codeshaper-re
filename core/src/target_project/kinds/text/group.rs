// Standard Uses
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

// Crate Uses
use crate::target_project::kinds::text::file::TextFile;
use crate::target_project::kinds::text::TextSolution;
use crate::target_project::{Group, Target};

// External Uses
use indextree::NodeId;


#[derive(PartialEq)]
pub struct TextGroup<'a> {
    pub name: String,
    // pub root: &'a RefCell<TextSolution<'a>>,
    pub parent: Option<NodeId>,
    pub groups: Vec<NodeId>,
    pub files: Vec<TextFile>,
    p: PhantomData<&'a TextGroup<'a>>
}

#[allow(unused)]
impl<'a> TextGroup<'a> {
    pub fn new(
        target: &mut Box<TextSolution<'a>>,
        parent_id: Option<NodeId>,
        name: String,
        groups: Vec<TextGroup<'a>>,
        files: Vec<TextFile>,
    ) -> Self {
        let mut sub_groups = vec![];

        for sub_group in groups {
            let id = target.graph().new_node(sub_group);

            if let Some(parent_id) = parent_id {
                use std::borrow::BorrowMut;
                id.append(parent_id, target.graph().borrow_mut())
            }

            sub_groups.push(id);
        }

        Self {
            name, parent: parent_id,
            groups: sub_groups, files,
            p: Default::default(),
        }
    }
}

impl<'a> Debug for TextGroup<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextGroup")
            .field("groups", &self.groups)
            .finish()
    }
}

#[allow(unused)]
impl<'a> Group for TextGroup<'a> {
    type Group = Self;
    type File = TextFile;

    fn name(&self) -> &String {
        &self.name
    }
    fn parent(&self) -> &Option<NodeId> { &self.parent }
    fn groups(&self) -> &Vec<NodeId> {
        &self.groups
    }
    fn files(&self) -> &Vec<Self::File> {
        &self.files
    }

    fn add_group(&mut self, group: Self::Group) -> NodeId {
        todo!()
    }
}
