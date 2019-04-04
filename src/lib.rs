pub mod scene_container;
pub use scene_container::SceneContainer;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

type NodeId = u64;

#[cfg(test)]
mod tests;
