extern crate mintree1;

use mintree1::*;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    // read filename from args
    let filename = env::args().nth(1).expect("Please provide a file name.");
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

    let mut graph: Adjacency = vec![vec![0.0; num_nodes]; num_nodes];
    for (line, _) in lines.zip(0..num_edges) {
        let line_str = line.expect("Cannot read line.");
        let words: Vec<&str> = line_str.split_whitespace().collect();

        let node1 = words[0].parse::<usize>().expect("Cannot parse edge 1.");
        let node2 = words[1].parse::<usize>().expect("Cannot parse edge 2.");
        let weight = words[2].parse::<f32>().expect("Cannot parse weight.");

        graph[node1][node2] = weight;
        graph[node2][node1] = weight;
    }
    // println!("{:?}", graph);

    let mst = mintree1::prim1::run(graph, num_nodes, 0);

    println!("{:?}", mst);
}
