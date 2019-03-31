use crate::scene_container::SceneNode;
use crate::SceneContainer;
use std::collections::HashMap;
use std::i128::*;

#[test]
fn add_get_node() {
    let mut container = SceneContainer::new();

    let foo = SceneNode::new(&mut container, Some("foo"));
    SceneNode::new(&mut container, None);
    let bar = SceneNode::new(&mut container, Some("bar"));
    SceneNode::new(&mut container, None);
    let baz = SceneNode::new(&mut container, Some("baz"));

    assert_eq!(foo, container.get_node(0).unwrap().borrow().get_id());
    assert_eq!(bar, container.get_node(2).unwrap().borrow().get_id());
    assert_eq!(baz, container.get_node(4).unwrap().borrow().get_id());
}

#[test]
fn add_rem_node() {
    let mut container = SceneContainer::new();

    let foo = SceneNode::new(&mut container, Some("foo"));
    assert_eq!(1, container.node_count());
    let bar = SceneNode::new(&mut container, Some("bar"));
    assert_eq!(2, container.node_count());
    let baz = SceneNode::new(&mut container, Some("baz"));
    assert_eq!(3, container.node_count());
    container.remove_node(foo);
    assert_eq!(2, container.node_count());
    container.remove_node(foo);
    assert_eq!(2, container.node_count());
    container.remove_node(bar);
    assert_eq!(1, container.node_count());
    container.remove_node(baz);
    assert_eq!(0, container.node_count());
}

#[test]
fn add_child() {
    let mut container = SceneContainer::new();

    let foo = SceneNode::new(&mut container, Some("foo"));
    let bar = SceneNode::new(&mut container, Some("bar"));

    let mut foo_node = container.get_node(foo).unwrap().borrow_mut();
    foo_node.add_child(bar);

    println!("{:?}", foo_node);
}
