use std::cmp::Ordering;

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
}
