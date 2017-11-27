use super::*;

trait HasIndex {
    fn get_index(&self) -> usize;
}


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
    let mut queue: MinHeap<Nodeish> = MinHeap::new(&nodes);
    // while queue is not empty do {
    while !queue.is_empty() {
        // node_u <- extract_min(queue)
        let Nodeish { i: u, key, .. } = queue.extract_min().expect("Queue won't be exhausted.");
        mst.push(Edge::new(u, 0, key));
        // for node_v in adj[node_u] {
        for (v, &weight_v) in adj[u].adjacency.iter().enumerate() {
            // if (node_v in queue) and adj[node_u][node_v] < key[node_v] {
            if queue.has(v) && (queue.is_empty() || weight_v < queue.peek_by_index(v).key) {
                // pi[node_v] = node_u; key[node_v] = adj[node_u][node_v];
                let new_nodeish = Nodeish {
                    i: v,
                    pi: Some(u),
                    key: weight_v,
                };
                if !queue.is_empty() {
                    queue.decrease_key(v, &new_nodeish);
                }
            }
        }
    }

    // return
    mst
}

fn parent(index: usize) -> usize {
    (index - 1) / 2
}

fn left(index: usize) -> usize {
    index * 2 + 1
}

fn right(index: usize) -> usize {
    index * 2 + 2
}

struct MinHeap<T> {
    data: Vec<T>,
    lookup: Vec<Option<usize>>,
}

impl<T: Clone + Ord + HasIndex> MinHeap<T> {
    // Self = T;
    fn new(vec: &Vec<T>) -> MinHeap<T> {
        let mut clone = MinHeap {
            data: vec.clone(),
            lookup: (0..vec.len()).map(|i| Some(i)).collect(),
        };
        clone.build_min_heap();
        clone
    }

    pub fn peek(&self, i: usize) -> &T {
        &self.data[i]
    }

    pub fn peek_by_index(&self, i: usize) -> &T {
        self.peek(self.lookup[i].expect("Should not ask for deleted index."))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
        let i_index = self.peek(i).get_index();
        let j_index = self.peek(j).get_index();
        self.lookup.swap(i_index, j_index);
    }

    fn min_heapify(&mut self, i: usize) {
        let l = left(i);
        let r = right(i);
        let mut largest;

        if l < self.len() && self.peek(l) < self.peek(i) {
            largest = l;
        } else {
            largest = i;
        }

        if r < self.len() && self.peek(r) < self.peek(largest) {
            largest = r;
        }

        if largest != i {
            self.swap(i, largest);
            self.min_heapify(largest);
        }
    }

    fn build_min_heap(&mut self) {
        for i in (0..self.len() / 2).rev() {
            self.min_heapify(i);
        }
    }

    pub fn extract_min(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            let ret = self.data.swap_remove(0);
            if !self.is_empty() {
                self.lookup[self.data[0].get_index()] = Some(0);
                self.lookup[ret.get_index()] = None;
            }
            self.min_heapify(0);
            Some(ret)
        }
    }

    pub fn decrease_key(&mut self, i: usize, key: &T) {
        let mut i = self.lookup[i].expect("Can't decrease already removed key.");
        self.data[i] = key.clone();
        while i > 0 && self.peek(parent(i)) > self.peek(i) {
            self.swap(i, parent(i));
            i = parent(i);
        }
    }

    pub fn has(&self, key: usize) -> bool {
        self.lookup[key].is_some()
    }
}