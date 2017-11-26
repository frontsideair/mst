use super::*;

pub fn run(graph: &Graph, _start_node: NodeID) -> Vec<Edge> {
    let edges = &graph.edges;
    let num_nodes = graph.nodes.len();
    // initialize mst to edges
    let mut adjacency_list = graph.nodes.clone();
    let mut mst_copy = edges.clone();
    let mut mst = Vec::with_capacity(num_nodes - 1);
    // sort edges
    mst_copy.sort();
    mst_copy.reverse();
    // for edges
    for edge in mst_copy {
        // remove heaviest edge
        adjacency_list[edge.left].adjacency[edge.right] = Weight::MAX;
        adjacency_list[edge.right].adjacency[edge.left] = Weight::MAX;
        // check is connected
        if !is_connected(&adjacency_list) {
            // if not connected, add edge back
            adjacency_list[edge.left].adjacency[edge.right] = edge.weight;
            adjacency_list[edge.right].adjacency[edge.left] = edge.weight;
            mst.push(edge);
        }
    }

    // return
    mst
}

fn is_connected(adjacency_list: &Vec<Node>) -> bool {
    let visited = dfs(&adjacency_list);
    visited.iter().all(|&item| item)
}

fn dfs_helper(adjacency_list: &Vec<Node>, start: NodeID, visited: &mut Vec<bool>) {
    let neighbors = &adjacency_list[start].adjacency;
    for (neighbor, weight) in neighbors.iter().enumerate() {
        if weight != &Weight::MAX && !visited[neighbor] {
            visited[neighbor] = true;
            dfs_helper(&adjacency_list, neighbor, visited);
        }
    }
}

fn dfs(adjacency_list: &Vec<Node>) -> Vec<bool> {
    let mut visited = vec![false; adjacency_list.len()];
    dfs_helper(&adjacency_list, 0, &mut visited);

    visited
}