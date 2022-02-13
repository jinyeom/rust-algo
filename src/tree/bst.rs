use std::cmp::Ordering;
use std::ops::Deref;

pub struct BST<T: Ord> {
    data: Option<T>,
    left: Option<Box<BST<T>>>,
    right: Option<Box<BST<T>>>,
}

impl<T> BST<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        BST {
            data: None,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, data: T) {
        if self.data.is_none() {
            self.data = Some(data);
        } else {
            match &self.data {
                None => (),
                // TODO: Can we use `unwrap` here instead?
                // https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
                Some(key) => {
                    let target = if data < *key {
                        &mut self.left
                    } else {
                        &mut self.right
                    };
                    match target {
                        None => {
                            let mut node = BST::new();
                            node.insert(data);
                            *target = Some(Box::new(node));
                        }
                        Some(ref mut node) => {
                            node.insert(data);
                        }
                    }
                }
            }
        }
    }

    // Currently only checks if the data exits in the tree;
    // Fix this so that the tree stores key value pairs, and
    // search() returns the value if the key exists; None otherwise.
    pub fn search(&self, data: &T) -> bool {
        match &self.data {
            None => false,
            Some(key) => match data.cmp(key) {
                Ordering::Equal => true,
                Ordering::Less => match &self.left {
                    None => false,
                    Some(node) => node.search(data),
                },
                Ordering::Greater => match &self.right {
                    None => false,
                    Some(node) => node.search(data),
                },
            },
        }
    }

    pub fn iter() {}
}

struct BSTIter<'a, T>
where
    T: Ord,
{
    stack: Vec<&'a BST<T>>,
}

impl<'a, T> BSTIter<'a, T>
where
    T: Ord,
{
    pub fn new(bst: &BST<T>) -> BSTIter<T> {
        let mut iter = BSTIter {
            stack: vec![bst],  // insert the root
        };
        iter.stack_push_left();
        iter
    }

    fn stack_push_left(&mut self) {
        while let Some(child) = &self.stack.last().unwrap().left {
            self.stack.push(child);
        }
    }
}

impl<'a, T> Iterator for BSTIter<'a, T>
where
    T: Ord,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.stack.is_empty() {
            None
        } else {
            let node = self.stack.pop().unwrap();
            if node.right.is_some() {
                self.stack.push(node.right.as_ref().unwrap().deref());
                self.stack_push_left();  // push left of the right child node we just pushed
            }
            node.data.as_ref()
        }
    }
}