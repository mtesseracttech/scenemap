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
    pub fn create_new(
        container: Rc<RefCell<SceneContainer>>,
        name: Option<&str>,
        parent: Option<u64>,
    ) -> u64 {
        let mut container_deref = (*container).borrow_mut();
        let new_node_id = container_deref.add_node(Rc::new(RefCell::new(SceneNode {
            container: container.clone(),
            parent,
            children: Vec::new(),
            id: 0,
            name: if name.is_some() {
                name.unwrap().to_string()
            } else {
                "n/a".to_string()
            },
        })));

        if parent.is_some() {
            let parent_id = parent.unwrap();
            let parent_node = container_deref.get_node(parent_id).unwrap();
            let mut parent_node = (*parent_node).borrow_mut();
            parent_node.add_child(new_node_id);
        }

        new_node_id
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

    pub fn add_child_id(&mut self, node_id: u64) {
        if (*self.container).borrow_mut().has_node(node_id) {
            match self.children.iter().find(|node| **node == node_id) {
                None => self.children.push(node_id),
                Some(_) => (),
            }
        }
    }

    pub fn add_child(&mut self, child_node: Rc<RefCell<SceneNode>>) {
        self.add_child_id((*child_node).borrow().get_id());
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

    //Gets a child from a given index, if the child was not available, it returns None
    pub fn get_child(&self, index: usize) -> Option<Rc<RefCell<SceneNode>>> {
        (*self.container)
            .borrow_mut()
            .get_node(self.children[index])
    }

    //Returns a list of child id's guaraneed to be available for usage from the data structure
    pub fn get_children_ids(&mut self) -> Vec<u64> {
        self.children
            .iter()
            .take_while(|c| (*self.container).borrow().has_node(**c))
            .cloned()
            .collect()
    }

    //Returns a list of children guaranteed to not be deleted from the data structure
    pub fn get_children(&self) -> Vec<Rc<RefCell<SceneNode>>> {
        self.children
            .iter()
            .take_while(|c| (*self.container).borrow().has_node(**c))
            .flat_map(|c| (*self.container).borrow_mut().get_node(*c))
            .collect()
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<SceneNode>>> {
        match self.parent {
            None => None,
            Some(parent_id) => (*self.container).borrow_mut().get_node(parent_id),
        }
    }

    pub fn set_parent_id(&mut self, parent_id: u64) {
        if (*self.container).borrow_mut().has_node(parent_id) {
            self.parent = Some(parent_id);
        }
    }

    pub fn get_parent_id(&self) -> Option<u64> {
        self.parent
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

    pub fn get_node(&self, node_id: u64) -> Option<Rc<RefCell<SceneNode>>> {
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

    pub fn has_node(&self, node_id: u64) -> bool {
        self.members.contains_key(&node_id)
    }

    pub fn node_count(&self) -> usize {
        self.members.len()
    }
}
