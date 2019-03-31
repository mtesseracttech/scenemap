pub mod scene_container;
pub use scene_container::SceneContainer;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::cell::RefCell;
use std::rc::Rc;


type NodeId = u64;

#[cfg(test)]
mod tests;
