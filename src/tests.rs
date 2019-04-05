use crate::scene_container::SceneNode;
use crate::SceneContainer;
use core::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

#[test]
fn add_get_node() {
    let container = SceneContainer::new();

    let foo = SceneNode::create_new(container.clone(), Some("foo"), None);
    SceneNode::create_new(container.clone(), None, None);
    let bar = SceneNode::create_new(container.clone(), Some("bar"), None);
    SceneNode::create_new(container.clone(), None, None);
    let baz = SceneNode::create_new(container.clone(), Some("baz"), None);

    assert_eq!(
        foo,
        (*container)
            .borrow_mut()
            .get_node(0)
            .unwrap()
            .borrow()
            .get_id()
    );
    assert_eq!(
        bar,
        (*container)
            .borrow_mut()
            .get_node(2)
            .unwrap()
            .borrow()
            .get_id()
    );
    assert_eq!(
        baz,
        (*container)
            .borrow_mut()
            .get_node(4)
            .unwrap()
            .borrow()
            .get_id()
    );
}

#[test]
fn add_rem_node() {
    let container = SceneContainer::new();

    let foo = SceneNode::create_new(container.clone(), Some("foo"), None);
    assert_eq!(1, container.borrow().node_count());
    let bar = SceneNode::create_new(container.clone(), Some("bar"), None);
    assert_eq!(2, container.borrow().node_count());
    let baz = SceneNode::create_new(container.clone(), Some("baz"), None);
    assert_eq!(3, container.borrow().node_count());
    (*container).borrow_mut().remove_node(foo);
    //container.borrow_mut().remove_node(foo);
    //container.remove_node(foo);
    assert_eq!(2, container.borrow().node_count());
    (*container).borrow_mut().remove_node(foo);
    //container.borrow_mut().remove_node(foo);
    //container.remove_node(foo);
    assert_eq!(2, container.borrow().node_count());
    (*container).borrow_mut().remove_node(bar);
    //container.borrow_mut().remove_node(bar);
    //container.remove_node(bar);
    assert_eq!(1, container.borrow().node_count());
    (*container).borrow_mut().remove_node(baz);
    //container.borrow_mut().remove_node(baz);
    //container.remove_node(baz);
    assert_eq!(0, container.borrow().node_count());
}

#[test]
fn add_child() {
    let container = SceneContainer::new();

    let foo_id = SceneNode::create_new(container.clone(), Some("foo"), None);
    let bar_id = SceneNode::create_new(container.clone(), Some("bar"), None);
    let foo = (*container).borrow_mut().get_node(foo_id).unwrap();
    (*foo).borrow_mut().add_child(bar_id);

    assert_eq!(1, (*foo).borrow().get_child_count());
    assert_eq!(
        "bar",
        (*(*foo).borrow_mut().get_child(0).unwrap())
            .borrow_mut()
            .get_name()
    );

    println!("{:?}", foo);
    let baz_id = SceneNode::create_new(container.clone(), Some("baz"), None);

    (*foo).borrow_mut().add_child(baz_id);

    assert_eq!(2, (*foo).borrow().get_child_count());
    assert_eq!(vec![bar_id, baz_id], (*foo).borrow_mut().get_children_ids());
    println!("{:?}", foo);
}

#[test]
fn remove_child() {
    let container = SceneContainer::new();

    let foo = SceneNode::create_new(container.clone(), Some("foo"), None);
    let bar = SceneNode::create_new(container.clone(), Some("bar"), None);
}
