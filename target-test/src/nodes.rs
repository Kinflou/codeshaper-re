#[allow(unused)]
enum Node<Item> {
    Leaf(Item),
    Children(Vec<Node<Item>>),
}

#[allow(unused)]
impl<It> Node<It> {
    fn iter(&self) -> NodeIter<'_, It> {
        NodeIter {
            children: std::slice::from_ref(self),
            parent: None,
        }
    }
}

struct NodeIter<'a, It> {
    children: &'a [Node<It>],
    parent: Option<Box<NodeIter<'a, It>>>,
}

impl<'a, It> Iterator for NodeIter<'a, It> {
    type Item = &'a It;

    fn next(&mut self) -> Option<Self::Item> {
        match self.children.first() {
            None => match self.parent.take() {
                Some(parent) => {
                    // continue with the parent node
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(Node::Leaf(item)) => {
                self.children = &self.children[1..];
                Some(item)
            }
            Some(Node::Children(children)) => {
                self.children = &self.children[1..];

                // start iterating the child trees
                *self = NodeIter {
                    children: children.as_slice(),
                    parent: Some(Box::new(std::mem::take(self))),
                };
                self.next()
            }
        }
    }
}

impl<It> Default for NodeIter<'_, It> {
    fn default() -> Self {
        NodeIter {
            children: &[],
            parent: None,
        }
    }
}

mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_borrowing_iterator() {
        let tree = Node::Children(vec![
            Node::Leaf(5),
            Node::Leaf(4),
            Node::Children(vec![Node::Leaf(3), Node::Leaf(2), Node::Children(vec![])]),
            Node::Children(vec![Node::Children(vec![
                Node::Children(vec![Node::Leaf(1)]),
                Node::Leaf(0),
            ])]),
        ]);

        let nums: Vec<i32> = tree.iter().copied().collect();
        assert_eq!(nums, vec![5, 4, 3, 2, 1, 0]);
    }
}
