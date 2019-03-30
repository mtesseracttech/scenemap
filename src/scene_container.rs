use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use core::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::net::Shutdown::Read;
use std::path::Iter;

#[derive(Debug)]
pub struct AssociatedNode{
    id : u64,
    name: String
}

#[derive(Debug)]
pub struct SceneNode{
    parent : Option<AssociatedNode>,
    children: Vec<AssociatedNode>,
    id : u64
}

impl SceneNode{
    pub fn new(container: &mut SceneContainer) -> u64{
        container.add_node(&mut Rc::new(RefCell::new(SceneNode{
            parent: None,
            children: vec![],
            id: 0
        })))
    }

    pub(in crate::scene_container) fn set_id(&mut self, id: u64){
        self.id = id;
    }

    pub fn get_id(&self) -> u64{
        self.id
    }
}

#[derive(Debug)]
pub struct SceneContainer{
    members: HashMap<u64, Rc<RefCell<SceneNode>>>,
    cur_id: u64
}

impl SceneContainer{
    pub fn new() -> SceneContainer{
        SceneContainer{
            members: HashMap::new(),
            cur_id: 0
        }
    }

    pub fn get_node(&mut self, node_id: u64) -> Option<&Rc<RefCell<SceneNode>>>{
        self.members.get(&node_id)
    }

    pub fn get_node_mut(&mut self, node_id: u64) -> Option<&mut Rc<RefCell<SceneNode>>>{
        self.members.get_mut(&node_id)
    }

    pub fn add_node(&mut self, node: &mut Rc<RefCell<SceneNode>>) -> u64{
        let node_id = self.cur_id;
        node.as_ref().borrow_mut().set_id(node_id);
        self.members.insert(self.cur_id, node.to_owned());
        self.cur_id += 1;
        node_id
    }

    pub fn len(&self) -> usize{
        self.members.len()
    }
}