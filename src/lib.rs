use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::u32;

pub mod prim1;

pub type Node = usize;

#[derive(Debug)]
pub struct Weight(u32);

impl Weight {
    pub fn new(num: f32) -> Weight {
        Weight((num * 100000.0) as u32)
    }

    pub const MAX: Weight = Weight(u32::MAX);
}

impl Clone for Weight {
    fn clone(&self) -> Weight {
        Weight(self.0.clone())
    }
}

impl Copy for Weight {}
impl Eq for Weight {}

impl PartialEq for Weight {
    fn eq(&self, other: &Weight) -> bool {
        other.0.eq(&self.0)
    }
}

impl Ord for Weight {
    fn cmp(&self, other: &Weight) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Weight) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Hash for Weight {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

pub type Adjacency = Vec<Vec<Weight>>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    left: Node,
    right: Node,
    pub weight: Weight,
}

// this ensures Edge::new(1,2) == Edge::new(2,1)
impl Edge {
    pub fn new(node1: Node, node2: Node, weight: Weight) -> Edge {
        if node1 < node2 {
            Edge {
                left: node1,
                right: node2,
                weight,
            }
        } else {
            Edge {
                left: node2,
                right: node1,
                weight,
            }
        }
    }
}