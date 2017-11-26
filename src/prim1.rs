use super::*;

pub fn run(adjacency_list: &Adjacency, num_nodes: usize, start_node: Node) -> Vec<Edge> {
    let mut nodes_added: Vec<bool> = vec![false; num_nodes];
    nodes_added[start_node] = true;
    let mut mst: Vec<Edge> = Vec::with_capacity(num_nodes - 1);

    for _ in 0..num_nodes - 1 {
        // add edge of min weight with one vertex in mst and one without
        let mut min = Weight::MAX;

        // assume these will always be initialized in the loop below
        let mut add_node: usize = 0;
        let mut edge = Edge::new(0, 0, Weight::MIN);

        for j in 0..num_nodes {
            if nodes_added[j] {
                for k in 0..num_nodes {
                    if !nodes_added[k] && (adjacency_list[j][k] < min) {
                        add_node = k;
                        edge = Edge::new(j, k, adjacency_list[j][k]);
                        min = adjacency_list[j][k];
                    }
                }
            }
        }
        nodes_added[add_node] = true;
        mst.push(edge);
    }

    // return
    mst
}