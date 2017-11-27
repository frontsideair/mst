use super::*;

fn parent(index: usize) -> usize {
    (index - 1) / 2
}

fn left(index: usize) -> usize {
    index * 2 + 1
}

fn right(index: usize) -> usize {
    index * 2 + 2
}

pub struct MinHeap<T> {
    data: Vec<T>,
    lookup: Vec<Option<usize>>,
}

impl<T: Clone + Ord + HasIndex> MinHeap<T> {
    pub fn new(vec: &Vec<T>) -> MinHeap<T> {
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