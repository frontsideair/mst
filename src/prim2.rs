use super::*;

pub fn run(graph: &Graph, start_node: NodeID) -> Vec<Edge> {
    let num_nodes = graph.nodes.len();
    let adjacency_list = &graph.nodes;

    let mut mst: Vec<Edge> = Vec::with_capacity(num_nodes - 1);
    let mut labels: Vec<Weight> = Vec::new();
    let mut back: Vec<NodeID> = Vec::new();
    let mut permanent: Vec<bool> = vec![false; num_nodes];

    for j in 0..num_nodes {
        labels.push(adjacency_list[start_node].adjacency[j]);
        back.push(start_node);
    }

    labels[start_node] = Weight::MIN;
    permanent[start_node] = true;

    for _ in 1..num_nodes {
        // take i of smallest weight
        let labels_clone = labels.clone();
        let (i, min) = labels_clone.iter().enumerate().fold(
            (0, &Weight::MAX),
            |acc, item| if !permanent[item.0] &&
                item.1 < acc.1
            {
                item
            } else {
                acc
            },
        );
        // make i permanent
        permanent[i] = true;
        // add edge(i, back[i]) to the mst
        mst.push(Edge::new(i, back[i], min.clone()));
        // for each temporary label k do
        for (k, weight) in labels_clone.iter().enumerate().filter(
            |item| !permanent[item.0],
        )
        {
            // if (adjacency[i,k] < labels[k]) then
            if &adjacency_list[i].adjacency[k] < weight {
                labels[k] = adjacency_list[i].adjacency[k];
                back[k] = i;
            }
        }
    }

    // return
    mst
}
