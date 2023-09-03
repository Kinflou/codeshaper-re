// Relative Modules
pub mod project;
pub mod file;
pub mod group;

// Standard Uses
use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::path::Path;
use std::rc::{Rc, Weak};

// Crate Uses
use crate::workspace::operation::Operation;
use crate::target_project::{Group, Target};
use crate::target_project::kinds::text::project::Project;
use crate::target_project::kinds::text::group::TextGroup;
use crate::target_project::iterator::TargetIter;

// External Uses
use eyre::Result;


pub struct TextSolution<'a> {
    pub name: String,
    pub groups: Vec<Weak<RefCell<dyn Group>>>,
    pub file_map: Option<TargetIter<'a>>
}

#[allow(unused)]
impl<'a> TextSolution<'a> {
    pub fn from_path_shared(path: &'a Path) -> Result<Rc<RefCell<dyn Target + '_>>> {
        Ok(Self::from_target(path)?)
    }

    pub fn from_target(path: &'a Path) -> Result<Rc<RefCell<TextSolution>>> {
        let project = Project::from_path(path)?;

        fn group_recursive(
            name: &String, group: &project::Group,
            text_group: Rc<RefCell<TextGroup>>
        ) -> Rc<RefCell<TextGroup>> {
            let mut solution_group = TextGroup::new_shared(name.to_string());

            for (name, project_group) in &group.groups {
                let children = group_recursive(
                    name, project_group, Rc::clone(&solution_group)
                );

                RefCell::borrow_mut(&solution_group).groups.push(
                    Rc::downgrade(&children) as _
                );
            }

            solution_group
        }

        let root_group = TextGroup::new_shared("root".to_string());
        let root_group = group_recursive(
            &"root".to_string(), &project.groups["root"], root_group
        );

        let mut solution = Self {
            name: project.name,
            groups: vec![Rc::downgrade(&root_group) as _],
            file_map: None
        };
        // solution.file_map = Some(solution.iter());

        Ok(Rc::new(RefCell::new(solution)))
    }

    pub fn new_shared(name: String) -> Rc<RefCell<TextSolution<'a>>> {
        Rc::new(RefCell::new(
            Self {
                name,
                groups: vec![],
                file_map: None
            }
        ))
    }

    pub fn add_group(&mut self, group: Weak<RefCell<dyn Group>>) {
        self.groups.push(group);
    }

}

#[allow(unused)]
impl Debug for TextSolution<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[allow(unused)]
impl<'a> Target for TextSolution<'a> {
    fn name(&self) -> &String { &self.name }
    fn groups(&self) -> &Vec<Weak<RefCell<dyn Group>>> { &self.groups }
    fn operation(&self) -> &Option<Operation> { todo!() }
    fn ast_set(&self) { todo!() }
    fn add_group(&mut self, group: Weak<RefCell<dyn Group>>) {
        self.groups.push(group);
    }

    fn file_map(&mut self) -> &mut Option<TargetIter<'_>> { &mut self.file_map }
}
