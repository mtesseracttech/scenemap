pub mod scene_container;
pub use scene_container::SceneContainer;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::cell::RefCell;
use std::rc::Rc;


type NodeId = u64;

#[cfg(test)]
mod tests {
    use crate::SceneContainer;
    use std::collections::HashMap;
    use crate::scene_container::SceneNode;

    #[test]
    fn add_get_node() {
        let mut container = SceneContainer::new();

        let node_lol = SceneNode::new(&mut container);
        SceneNode::new(&mut container);
        let node_foo = SceneNode::new(&mut container);
        SceneNode::new(&mut container);
        let node_bar = SceneNode::new(&mut container);

        assert_eq!(node_lol, container.get_node(0).unwrap().borrow().get_id());
        assert_eq!(node_foo, container.get_node(2).unwrap().borrow().get_id());
        assert_eq!(node_bar, container.get_node(4).unwrap().borrow().get_id());
    }
}
