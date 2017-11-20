use super::*;

pub fn run(adjacency_list: Adjacency, num_nodes: usize, start_node: Node) -> HashSet<Edge> {
    let mut nodes_added: Vec<bool> = vec![false; num_nodes];
    nodes_added[start_node] = true;
    let mut mst: HashSet<Edge> = HashSet::new();

    for _ in 0..num_nodes - 1 {
        // add edge of min weight with one vertex in mst and one without
        let mut min = f32::MAX;

        // assume these will always be initialized in the loop below
        let mut add_node: usize = 0;
        let mut edge = Edge { left: 0, right: 0 };

        for j in 0..num_nodes {
            if nodes_added[j] {
                for k in 0..num_nodes {
                    if !nodes_added[k] && adjacency_list[j][k] < min {
                        add_node = k;
                        edge = Edge { left: j, right: k };
                        min = adjacency_list[j][k];
                    }
                }
            }
        }
        nodes_added[add_node] = true;
        mst.insert(edge);
    }

    // return
    mst
}