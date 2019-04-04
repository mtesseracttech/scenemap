use crate::scene_container::SceneNode;
use crate::SceneContainer;
use core::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn add_get_node() {
    let container = SceneContainer::new();

    let foo = SceneNode::create_new(container.clone(), Some("foo"));
    SceneNode::create_new(container.clone(), None);
    let bar = SceneNode::create_new(container.clone(), Some("bar"));
    SceneNode::create_new(container.clone(), None);
    let baz = SceneNode::create_new(container.clone(), Some("baz"));

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

    let foo = SceneNode::create_new(container.clone(), Some("foo"));
    assert_eq!(1, container.borrow().node_count());
    let bar = SceneNode::create_new(container.clone(), Some("bar"));
    assert_eq!(2, container.borrow().node_count());
    let baz = SceneNode::create_new(container.clone(), Some("baz"));
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

    let foo = SceneNode::create_new(container.clone(), Some("foo"));
    let bar = SceneNode::create_new(container.clone(), Some("bar"));

    let foo_node = (*container).borrow_mut().get_node(foo).unwrap();
    (*foo_node).borrow_mut().add_child(bar);

    assert_eq!(1, (*foo_node).borrow_mut().get_child_count());
    assert_eq!(
        "bar",
        (*(*foo_node).borrow_mut().get_child(0).unwrap())
            .borrow_mut()
            .get_name()
    );

    println!("{:?}", foo_node);
}
