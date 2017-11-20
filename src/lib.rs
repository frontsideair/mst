use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::f32;

pub mod prim1;

pub type Node = usize;
pub type Weight = f32;
#[derive(Debug)]
pub struct Edge {
    pub left: Node,
    pub right: Node,
    // weight: Weight,
}
impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.left.hash(state);
        self.right.hash(state);
    }
}
impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.left == other.left && self.right == other.right
    }
}
impl Eq for Edge {}
pub type Adjacency = Vec<Vec<Weight>>;
