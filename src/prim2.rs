use super::*;

pub fn run(adjacency_list: Adjacency, num_nodes: usize, start_node: Node) -> Vec<Edge> {
    let mut mst: Vec<Edge> = Vec::with_capacity(num_nodes - 1);
    let mut labels: Vec<Weight> = Vec::new();
    let mut back: Vec<Node> = Vec::new();
    let mut permanent: Vec<bool> = Vec::new();

    for j in 0..num_nodes {
        labels.push(adjacency_list[start_node][j]);
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
            if &adjacency_list[i][k] < weight {
                labels[k] = adjacency_list[i][k];
                back[k] = i;
            }
        }
    }

    // return
    mst
}
