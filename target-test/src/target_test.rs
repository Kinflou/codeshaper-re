use downcast_rs::Downcast;
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

use crate::target::{File, Group, Target, TargetIter};

#[derive(Debug)]
pub struct TextTarget {
    root_group: Rc<RefCell<TextGroup>>,
    map: Option<TargetIter>,
}
impl Target for TextTarget {
    fn root_group(&self) -> &Rc<RefCell<dyn Group>> {
        &(self.root_group as _)
    }

    fn map(&self) -> &Option<TargetIter> {
        &self.map
    }

    fn refresh_state(&mut self) {
        self.map = Some(self.iter());
    }
}

#[derive(Debug)]
pub struct TextGroup {
    target: Weak<RefCell<TextTarget>>,
    parent: Option<Weak<RefCell<TextGroup>>>,
    groups: Vec<Rc<RefCell<TextGroup>>>,
    files: Vec<Rc<RefCell<TextFile>>>,
}
impl TextGroup {
    fn any_like(&self) -> &dyn Any {
        self
    }
}
impl Group for TextGroup {
    fn groups(&self) -> &Vec<Rc<RefCell<dyn Group>>> {
        todo!()
    }

    fn add_group(&mut self, group: Rc<RefCell<dyn Group>>) {
        let group_ref = group.borrow_mut();
        let grp = RefMut::map(group_ref, |r| r.downcast_mut().unwrap()) as _;

        self.groups.push(grp);
    }

    fn files(&self) -> &Vec<Rc<RefCell<dyn File>>> {
        todo!()
    }
}

#[derive(Debug)]
pub struct TextFile {
    parent: Weak<RefCell<TextGroup>>,
    name: String,
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
        let target = Rc::new_cyclic(|target| {
            RefCell::new(TextTarget {
                root_group: Rc::new_cyclic(|root_group| {
                    RefCell::new(TextGroup {
                        target: target.clone(),
                        parent: None,
                        groups: vec![
                            Rc::new_cyclic(|group2| {
                                RefCell::new(TextGroup {
                                    target: target.clone(),
                                    parent: Some(root_group.clone()),
                                    groups: vec![],
                                    files: vec![
                                        Rc::new_cyclic(|file| {
                                            RefCell::new(TextFile {
                                                parent: root_group.clone(),
                                                name: "File Three".to_string(),
                                            })
                                        }),
                                        Rc::new_cyclic(|file| {
                                            RefCell::new(TextFile {
                                                parent: root_group.clone(),
                                                name: "File Four".to_string(),
                                            })
                                        }),
                                    ],
                                })
                            }),
                            Rc::new_cyclic(|group3| {
                                RefCell::new(TextGroup {
                                    target: target.clone(),
                                    parent: Some(root_group.clone()),
                                    groups: vec![],
                                    files: vec![
                                        Rc::new_cyclic(|file| {
                                            RefCell::new(TextFile {
                                                parent: root_group.clone(),
                                                name: "File Five".to_string(),
                                            })
                                        }),
                                        Rc::new_cyclic(|file| {
                                            RefCell::new(TextFile {
                                                parent: root_group.clone(),
                                                name: "File SIx".to_string(),
                                            })
                                        }),
                                    ],
                                })
                            }),
                        ],
                        files: vec![
                            Rc::new_cyclic(|file| {
                                RefCell::new(TextFile {
                                    parent: root_group.clone(),
                                    name: "File One".to_string(),
                                })
                            }),
                            Rc::new_cyclic(|file| {
                                RefCell::new(TextFile {
                                    parent: root_group.clone(),
                                    name: "File Two".to_string(),
                                })
                            }),
                        ],
                    })
                }),
                map: None,
            })
        });
        let mut target_mut = RefCell::borrow_mut(&target);
        target_mut.refresh_state();

        // println!("{target_mut:#?}");

        let mut map = target_mut.map.as_mut().unwrap();

        println!("{:?}", map.next().unwrap());
        println!("{:?}", map.next().unwrap());
        println!("{:?}", map.next().unwrap());
        println!("{:?}", map.next().unwrap());
        println!("{:?}", map.next().unwrap());
        println!("{:?}", map.next().unwrap());
    }
}
