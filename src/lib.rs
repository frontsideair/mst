use std::cmp::{Ordering, min, max};
use std::u32;
use std::ops::Add;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub mod prim1;
pub mod prim2;

pub type Node = usize;

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Weight(u32);

impl Weight {
    pub fn new(num: f32) -> Weight {
        Weight((num * 100000.0) as u32)
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

pub fn read_graph_from_file(filename: &str) -> (Adjacency, Vec<Edge>, usize) {
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

    let mut graph: Adjacency = vec![vec![Weight::MAX; num_nodes]; num_nodes];
    let mut edges: Vec<Edge> = Vec::with_capacity(num_edges);
    for (line, _) in lines.zip(0..num_edges) {
        let line_str = line.expect("Cannot read line.");
        let words: Vec<&str> = line_str.split_whitespace().collect();

        let node1 = words[0].parse::<usize>().expect("Cannot parse edge 1.");
        let node2 = words[1].parse::<usize>().expect("Cannot parse edge 2.");
        let weight = Weight::new(words[2].parse::<f32>().expect("Cannot parse weight."));

        graph[node1][node2] = weight;
        graph[node2][node1] = weight;
        edges.push(Edge::new(node1, node2, weight));
    }

    (graph, edges, num_nodes)
}