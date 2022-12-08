use super::*;

pub struct Node {
    pub is_directory: bool,
    pub name: String,
    pub size: usize,
    pub contents: Vec<MagicNode>,
    pub parent: Option<MagicNode>,
}

impl Node {
    pub fn create_file(name: &str, size: usize) -> MagicNode {
        Rc::new(RefCell::new(Self {
            is_directory: false,
            name: String::from(name),
            size,
            contents: vec![],
            parent: None,
        }))
    }

    pub fn create_directory(name: &str) -> MagicNode {
        Rc::new(RefCell::new(Self {
            is_directory: true,
            name: String::from(name),
            size: 0,
            contents: vec![],
            parent: None,
        }))
    }

    pub fn add_size(&mut self, size: usize) {
        self.size += size;

        if let Some(parent) = &self.parent {
            parent.borrow_mut().add_size(size);
        }
    }
}
