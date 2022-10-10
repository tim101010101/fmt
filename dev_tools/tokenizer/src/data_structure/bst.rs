use std::fmt::Debug;

pub type Link<T, U> = Option<Box<BST<T, U>>>;

#[allow(dead_code)]
pub struct BST<T, U> {
    key: Option<T>,
    val: Option<U>,
    left: Link<T, U>,
    right: Link<T, U>,
}

#[allow(dead_code)]
impl<T, U> BST<T, U>
where
    T: Clone + Ord + Debug,
    U: Clone + Debug,
{
    pub fn new() -> Self {
        BST {
            key: None,
            val: None,
            left: None,
            right: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    pub fn len(&self) -> usize {
        self.calc_len(0)
    }

    fn calc_len(&self, mut i: usize) -> usize {
        if self.key.is_some() {
            i += 1;

            if !self.left.is_none() {
                i = self.left.as_ref().unwrap().calc_len(i);
            }
            if !self.right.is_none() {
                i = self.right.as_ref().unwrap().calc_len(i);
            }
        }

        i
    }

    pub fn insert(&mut self, key: T, val: U) {
        if self.key.is_none() {
            self.key = Some(key);
            self.val = Some(val);
        } else {
            match &self.key {
                Some(k) => {
                    if key == *k {
                        self.val = Some(val);
                        return;
                    }

                    let child = if key < *k {
                        &mut self.left
                    } else {
                        &mut self.right
                    };

                    match child {
                        Some(ref mut node) => node.insert(key, val),
                        None => {
                            let mut node = BST::new();
                            node.insert(key, val);
                            *child = Some(Box::new(node));
                        }
                    }
                }
                None => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_bst() {
        let b = BST::<u32, char>::new();
        assert_eq!(b.len(), 0)
    }

    #[test]
    fn bst_insert() {
        let mut b = BST::<u32, char>::new();
        assert_eq!(b.len(), 0);
        b.insert(3, 'a');
        assert_eq!(b.len(), 1);
        b.insert(9, 'v');
        assert_eq!(b.len(), 2)
    }
}
