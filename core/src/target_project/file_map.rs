// Standard Uses

// Crate Uses

// External Uses


/*
#[allow(unused)]
#[derive(Debug)]
pub struct FileMap {
    target: Rc<dyn Target>,
    selected_group: Option<Weak<RefCell<dyn Group>>>,
    selected_file_index: usize
}

impl FileMap {
    pub fn with_target(target: Rc<&dyn Target>) -> Self {
        Self {
            target,
            selected_group: None,
            selected_file_index: 0,
        }
    }

    /*
    pub fn from_target(target: Rc<RefCell<dyn Target>>) -> Self {
        Self {
            target,
            selected_group: None,
            selected_file_index: 0,
        }
    }
    */

    /*
    fn chain_file_iters<'a>(&mut self, group: Ref<dyn Group>)
        -> Chain<Iter<'a, Weak<dyn File>>, Iter<'a, Weak<dyn File>>> {
        let mut chains: Chain<Iter<Weak<dyn File>>, Iter<Weak<dyn File>>> = Chain::default();

        for group_inner in group.groups() {
            let gi = group_inner.upgrade().unwrap().borrow();
            chains = chains.chain(gi.files());
        }

        chains
    }
    */
}

impl Iterator for FileMap {
    type Item = Weak<RefCell<dyn File>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(group) = &self.selected_group {
            if let Some(_) = group.upgrade() {
            } else {
                if let Some(first) = self.target.groups().first() {
                    self.selected_group = Some(Weak::clone(first));
                    self.selected_file_index = 0;
                } else {
                    return None
                };
            }
        } else {
            if let Some(first) = self.target.groups().first() {
                self.selected_group = Some(Weak::clone(first));
            } else {
                return None
            };
        };

        if let Some(group) = &self.selected_group {
          if let Some(group) = group.upgrade() {
              let group = group.borrow();

              let files = &group.files();

              let Some(next_file) =
                  files.iter().nth(self.selected_file_index +1) else {
                  return None
              };

              let next_file = Weak::clone(next_file);

              self.selected_file_index += 1;

              return Some(next_file)
          };
        };

        return None
    }
}
*/
