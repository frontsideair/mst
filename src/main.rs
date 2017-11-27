extern crate mintree1;

use mintree1::*;

use std::env;

fn main() {
    // read filename from args
    let filename = env::args().nth(1).expect("Please provide a file name.");

    let graph = read_graph_from_file(&filename[..]);
    // println!("{:?}", graph);

    let mst = mintree1::prim4::run(&graph, 0);
    let sum: Weight = mst.iter().fold(Weight::MIN, |acc, edge| acc + edge.weight);

    println!("Total weight: {:?}", sum);
    println!("Number of edges: {}", mst.len());
}
