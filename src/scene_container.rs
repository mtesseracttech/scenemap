use core::fmt;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct SceneNode {
    container: Rc<RefCell<SceneContainer>>,
    parent: Option<u64>,
    children: Vec<u64>,
    id: u64,
    name: String,
}

impl SceneNode {
    pub fn create_new(container: Rc<RefCell<SceneContainer>>, name: Option<&str>) -> u64 {
        (*container)
            .borrow_mut()
            .add_node(Rc::new(RefCell::new(SceneNode {
                container: container.clone(),
                parent: None,
                children: Vec::new(),
                id: 0,
                name: if name.is_some() {
                    name.unwrap().to_string()
                } else {
                    "n/a".to_string()
                },
            })))
    }

    pub(in crate::scene_container) fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn add_child(&mut self, node_id: u64) {
        match self.children.iter().find(|node| **node == node_id) {
            None => self.children.push(node_id),
            Some(_) => (),
        }
    }

    pub fn remove_child(&mut self, node_id: u64) {
        let position = self.children.iter().position(|node| *node == node_id);
        match position {
            None => (),
            Some(index) => {
                self.children.remove(index);
            }
        }
    }

    pub fn get_child_count(&self) -> usize {
        self.children.len()
    }

    pub fn get_child(&self, index: usize) -> Option<Rc<RefCell<SceneNode>>> {
        (*self.container)
            .borrow_mut()
            .get_node(self.children[index])
    }
}

impl fmt::Debug for SceneNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {:?}, name: {:?}, parent: {:?}, children: {:?}",
            self.id, self.name, self.parent, self.children
        )
    }
}

#[derive(Debug)]
pub struct SceneContainer {
    members: HashMap<u64, Rc<RefCell<SceneNode>>>,
    cur_id: u64,
}

impl SceneContainer {
    pub fn new() -> Rc<RefCell<SceneContainer>> {
        Rc::new(RefCell::new(SceneContainer {
            members: HashMap::new(),
            cur_id: 0,
        }))
    }

    pub fn get_node(&mut self, node_id: u64) -> Option<Rc<RefCell<SceneNode>>> {
        //self.members.get(&node_id)
        Some(self.members.get(&node_id)?.clone())
    }

    pub fn add_node(&mut self, node: Rc<RefCell<SceneNode>>) -> u64 {
        let node_id = self.cur_id;
        node.as_ref().borrow_mut().set_id(node_id);
        self.members.insert(self.cur_id, node.to_owned());
        self.cur_id += 1;
        node_id
    }

    pub fn remove_node(&mut self, node_id: u64) {
        self.members.remove(&node_id);
    }

    pub fn node_count(&self) -> usize {
        self.members.len()
    }
}
