use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Node {
    is_word: bool,
    next: HashMap<char, Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(is_word: bool) -> Self {
        Node {
            is_word,
            next: HashMap::new(),
        }
    }
}

pub struct Trie {
    root: Rc<RefCell<Node>>,
    size: usize,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Rc::new(RefCell::new(Node::new(false))),
            size: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, word: String) {
        let mut temp = self.root.clone();
        for c in word.chars() {
            let n = temp.borrow().next.get(&c).cloned();
            if let Some(node) = n {
                temp = node.clone();
            } else {
                let t = Rc::new(RefCell::new(Node::new(false)));
                temp.borrow_mut().next.insert(c, t.clone());
                temp = t.clone();
            }
        }

        if temp.borrow().is_word == false {
            temp.borrow_mut().is_word = true;
            self.size += 1;
        }
    }
}
