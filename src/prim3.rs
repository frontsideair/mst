use super::*;

pub fn run(graph: &Graph, start_node: NodeID) -> Vec<Edge> {
    // initialize mst to edges
    let mut nodes = graph.nodes.clone(); // linear
    let mut edges = graph.edges.clone(); // linear
    let mut mst = Vec::with_capacity(graph.nodes.len() - 1);
    // sort edges
    edges.sort(); // linear
    edges.reverse(); // linear
    // for edges
    for edge in edges {
        // remove heaviest edge
        nodes[edge.left].adjacency[edge.right] = Weight::MAX;
        nodes[edge.right].adjacency[edge.left] = Weight::MAX;
        // check is connected
        if !is_connected(&nodes, start_node) {
            // if not connected, add edge back
            nodes[edge.left].adjacency[edge.right] = edge.weight;
            nodes[edge.right].adjacency[edge.left] = edge.weight;
            mst.push(edge);
        }
    }

    // return
    mst
}

fn is_connected(adjacency_list: &Vec<Node>, start_node: NodeID) -> bool {
    dfs(&adjacency_list, start_node) == adjacency_list.len()
}

fn dfs(adjacency_list: &Vec<Node>, start_node: NodeID) -> usize {
    let mut visited = vec![false; adjacency_list.len()]; // allocation
    let num_visited = dfs_helper(&adjacency_list, start_node, &mut visited, 0);

    num_visited
}

fn dfs_helper(
    adjacency_list: &Vec<Node>,
    start: NodeID,
    visited: &mut Vec<bool>,
    num_visited: usize,
) -> usize {
    let mut num_visited_copy = num_visited;
    let neighbors = &adjacency_list[start].adjacency;
    for (neighbor, weight) in neighbors.iter().enumerate() {
        if weight != &Weight::MAX && !visited[neighbor] {
            visited[neighbor] = true;
            num_visited_copy += 1 + dfs_helper(&adjacency_list, neighbor, visited, num_visited);
        }
    }
    num_visited_copy
}