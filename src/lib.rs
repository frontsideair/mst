use std::cmp::{Ordering, min, max};
use std::u32;
use std::ops::Add;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fmt::{Display, Formatter, Result};

pub mod prim1;
pub mod prim2;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn with_capacity(num_nodes: usize, num_edges: usize) -> Graph {
        Graph {
            nodes: vec![Node::with_capacity(num_nodes); num_nodes],
            edges: Vec::with_capacity(num_edges),
        }
    }

    fn add_edge(&mut self, left_index: usize, right_index: usize, w: f32) {
        let weight = Weight::new(w);
        self.edges.push(Edge::new(left_index, right_index, weight));

        self.nodes[left_index].adjacency[right_index] = weight;
        self.nodes[left_index].neighbors.push(right_index);

        self.nodes[right_index].adjacency[left_index] = weight;
        self.nodes[right_index].neighbors.push(left_index);
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Nodes: {}, Edges: {}",
            self.nodes.len(),
            self.edges.len()
        )
}
}

#[derive(Debug, Clone)]
struct Node {
    adjacency: Vec<Weight>,
    neighbors: Vec<NodeID>,
}

impl Node {
    fn with_capacity(capacity: usize) -> Node {
        Node {
            adjacency: vec![Weight::MAX; capacity],
            neighbors: Vec::with_capacity(capacity),
        }
    }
}

#[derive(Debug)]
pub struct Edge {
    left: NodeID,
    right: NodeID,
    pub weight: Weight,
}

impl Edge {
    fn new(left: NodeID, right: NodeID, weight: Weight) -> Edge {
        Edge {
            left: min(left, right),
            right: max(left, right),
            weight,
        }
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.left.eq(&other.left) && self.right.eq(&other.right)
    }
}

impl Eq for Edge {}

type NodeID = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Weight(u32);

impl Weight {
    pub fn new(num: f32) -> Weight {
        Weight((num * 100_000.0) as u32)
    }

    pub const MIN: Weight = Weight(0);
    pub const MAX: Weight = Weight(u32::MAX);
}

impl Add for Weight {
    type Output = Weight;

    fn add(self, other: Weight) -> Weight {
        Weight(self.0 + other.0)
    }
}

pub fn read_graph_from_file(filename: &str) -> Graph {
    let file = File::open(filename).expect("File could not be opened.");
    let mut lines = BufReader::new(file).lines();
    let num_nodes: usize = lines
        .next()
        .expect("Empty file.")
        .expect("Error reading file.")
        .parse()
        .expect("Not a number.");
    let num_edges: usize = lines
        .next()
        .expect("Empty file.")
        .expect("Error reading file.")
        .parse()
        .expect("Not a number.");

    let mut graph = Graph::with_capacity(num_nodes, num_edges);

    for (line, _) in lines.zip(0..num_edges) {
        let line_str = line.expect("Cannot read line.");
        let words: Vec<&str> = line_str.split_whitespace().collect();

        let node1 = words[0].parse::<usize>().expect("Cannot parse edge 1.");
        let node2 = words[1].parse::<usize>().expect("Cannot parse edge 2.");
        let weight = words[2].parse::<f32>().expect("Cannot parse weight.");

        graph.add_edge(node1, node2, weight);
    }

    graph
}