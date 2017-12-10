use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Nodeish {
    i: NodeID,
    pi: Option<NodeID>,
    key: Weight,
}

impl Nodeish {
    fn new(i: NodeID) -> Nodeish {
        Nodeish {
            i,
            pi: None,
            key: Weight::MAX,
        }
    }
}

impl HasIndex for Nodeish {
    fn get_index(&self) -> usize {
        self.i
    }
}

impl PartialOrd for Nodeish {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl Ord for Nodeish {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

pub fn run(graph: &Graph, start_node: NodeID) -> Vec<Edge> {
    let mut mst = Vec::new();
    let adj = &graph.nodes;
    let num_nodes = graph.nodes.len();
    let mut nodes: Vec<Nodeish> = (0..num_nodes).map(|i| Nodeish::new(i)).collect();
    // initialize key[start] to 0
    nodes[start_node].key = Weight::MIN;
    // initialize a priority queue of nodes
    let mut queue = minheap::MinHeap::new(&nodes);
    // while queue is not empty do {
    while !queue.is_empty() {
        // node_u <- extract_min(queue)
        let Nodeish { i: u, key, .. } = queue.extract_min().expect("Queue won't be exhausted.");
        mst.push(Edge::new(u, 0, key));
        // for node_v in adj[node_u] {
        for (v, &weight_v) in adj[u].adjacency.iter().enumerate() {
            // if (node_v in queue) and adj[node_u][node_v] < key[node_v] {
            if queue.has(v) && (queue.is_empty() || weight_v < queue.peek_by_index(v).key) {
                if !queue.is_empty() {
                    // pi[node_v] = node_u; key[node_v] = adj[node_u][node_v];
                    // potential improvement, pass the weight as key
                    let new_nodeish = Nodeish {
                        i: v,
                        pi: Some(u),
                        key: weight_v,
                    };
                    queue.decrease_key(v, &new_nodeish);
                }
            }
        }
    }

    // return
    mst
}