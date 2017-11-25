use std::cmp::{Ordering, min, max};
use std::u32;

pub mod prim1;
pub mod prim2;

pub type Node = usize;

#[derive(Debug)]
pub struct Weight(u32);

impl Weight {
    pub fn new(num: f32) -> Weight {
        Weight((num * 100000.0) as u32)
    }

    pub const MIN: Weight = Weight(0);
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
        self.0.eq(&other.0)
    }
}

impl Ord for Weight {
    fn cmp(&self, other: &Weight) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Weight {
    fn partial_cmp(&self, other: &Weight) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

pub type Adjacency = Vec<Vec<Weight>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Edge {
    left: Node,
    right: Node,
    pub weight: Weight,
}

// this ensures Edge::new(1,2) == Edge::new(2,1)
impl Edge {
    pub fn new(left: Node, right: Node, weight: Weight) -> Edge {
        Edge {
            left: min(left, right),
            right: max(left, right),
            weight,
        }
    }
}

impl Ord for Edge {
    fn cmp(&self, &other: &Edge) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, &other: &Edge) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}