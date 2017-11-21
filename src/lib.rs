use std::collections::HashSet;
use std::f32;

pub mod prim1;

pub type Node = usize;
pub type Weight = f32;
pub type Adjacency = Vec<Vec<Weight>>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    left: Node,
    right: Node,
    // weight: Weight,
}

// this ensures Edge::new(1,2) == Edge::new(2,1)
impl Edge {
    pub fn new(node1: Node, node2: Node) -> Edge {
        if node1 < node2 {
            Edge {
                left: node1,
                right: node2,
            }
        } else {
            Edge {
                left: node2,
                right: node1,
            }
        }
    }
}