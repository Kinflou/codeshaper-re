// Standard Uses
use std::cell::RefCell;
use std::rc::Rc;

// Crate Uses
use crate::target::{File, Group, Target};

// External Uses
use indextree::{Arena, NodeId};


#[derive(Debug)]
pub struct TextTarget {
    root_group: Option<NodeId>,
    graph: Arena<TextGroup>,
}

impl TextTarget {
    pub(crate) fn new<F>(root: F) -> Rc<RefCell<TextTarget>>
        where F: FnOnce(Rc<RefCell<TextTarget>>) -> NodeId
    {
        let graph = Arena::new();
        let target = Rc::new(RefCell::new(
            Self { root_group: None, graph }
        ));

        let root_group = root(target.clone());
        let mut target_mut = target.borrow_mut();
        target_mut.root_group = Some(root_group);

        drop(target_mut);

        target
    }
}

impl Target for TextTarget {
    fn root_group(&self) -> Option<NodeId> {
        self.root_group
    }

    /*
    fn map(&self) -> Option<&TargetIter> {
        &self.map
    }
    */

    fn refresh_state(&mut self) {
        todo!()
        // self.map = Some(self.iter());
    }
}

#[derive(Debug)]
pub struct TextGroup {
    target: Rc<RefCell<TextTarget>>,
    parent: Option<NodeId>,
    pub(crate) groups: Vec<NodeId>,
    pub(crate) files: Vec<TextFile>,
}

impl TextGroup {
    pub(crate) fn new<F>(
        target: Rc<RefCell<TextTarget>>, group: F, parent: Option<NodeId>
    ) -> NodeId
        where F: FnOnce(NodeId) -> (Vec<NodeId>, Vec<TextFile>)
    {
        let this = Self {
            target: target.clone(),
            parent,
            groups: vec![],
            files: vec![],
        };
        let mut target_mut = target.borrow_mut();
        let this_id = target_mut.graph.new_node(this);

        if let Some(parent) = parent {
            parent.append(this_id, &mut target_mut.graph)
        }

        drop(target_mut);

        let (sub_groups, files) = group(this_id);
        let mut target_mut = target.borrow_mut();
        let added_group = target_mut.graph.get_mut(this_id).unwrap().get_mut();

        for sub in sub_groups { added_group.groups.push(sub); }
        added_group.files = files;

        this_id
    }
}

impl Group<TextFile> for TextGroup {
    fn target(&self) -> Rc<RefCell<dyn Target>> {
        Rc::clone(&self.target) as _
    }

    fn files(&self) -> &Vec<TextFile> {
        &self.files
    }
}

#[derive(Debug)]
pub struct TextFile {
    target: Rc<RefCell<TextTarget>>,
    parent: NodeId,
    name: String,
}

impl TextFile {
    pub(crate) fn new(
        target: Rc<RefCell<TextTarget>>, group_id: NodeId, name: String
    ) -> Self {
        Self { target, parent: group_id, name, }
    }
}

impl File for TextFile {
    fn name(&self) -> &String {
        &self.name
    }
}

mod tests {
    use super::*;
    use std::cell::RefCell;

    #[allow(unused)]
    #[test]
    fn test_map() {
        let target =
            TextTarget::new(|target| {
                TextGroup::new(target.clone(), |root_group| {
                    (vec![
                        TextGroup::new(target.clone(), |root_group| {
                            (vec![], vec![
                                TextFile::new(
                                    target.clone(), root_group, "One".to_owned()
                                ),
                                TextFile::new(
                                    target.clone(), root_group, "Two".to_owned()
                                )
                            ])
                        }, Some(root_group))
                    ], vec![
                        TextFile::new(
                            target.clone(), root_group, "Two".to_owned()
                        )
                    ])
                }, None)
            });

        let mut target_mut = RefCell::borrow_mut(&target);

        // target_mut.refresh_state();

        // println!("{target_mut:#?}");

        for e in target_mut.graph.iter() {
            println!("{:?}", e.get().files());
        }

        /*
        let mut map = target_mut.map.as_mut().unwrap();

        println!("{:?}", map.next().unwrap());
        println!("{:?}", map.next().unwrap());
        println!("{:?}", map.next().unwrap());
        println!("{:?}", map.next().unwrap());
        println!("{:?}", map.next().unwrap());
        println!("{:?}", map.next().unwrap());
        */
    }
}
