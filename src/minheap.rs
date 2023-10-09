pub mod minheap {

    pub struct Heap {
        heap_vec: Vec<i32>,
        pointer_vec: Vec<usize>,
    }

    fn left_child(index: usize) -> usize {
        2 * index + 1
    }

    fn right_child(index: usize) -> usize {
        2 * index + 2
    }

    fn parent(index: usize) -> usize {
        (index - 1) / 2
    }

    pub fn initialize() -> Heap {
        Heap {
            heap_vec: vec![],
            pointer_vec: vec![],
        }
    }

    impl Heap {
        pub fn is_empty(&self) -> bool {
            self.heap_vec.len() == 0
        }

        pub fn push(&mut self, key: i32, value: usize) {
            self.heap_vec.push(key);
            self.pointer_vec.push(value);
            let index = self.heap_vec.len() - 1;
            self.sift_up(index);
        }

        pub fn pop(&mut self) -> (i32, usize) {
            self.swap(0, self.heap_vec.len() - 1);
            let ret = (
                self.heap_vec.pop().unwrap(),
                self.pointer_vec.pop().unwrap(),
            );
            self.sift_down(0);
            ret
        }

        fn swap(&mut self, index_a: usize, index_b: usize) {
            let temp1 = self.heap_vec[index_a];
            let temp2 = self.pointer_vec[index_a];
            self.heap_vec[index_a] = self.heap_vec[index_b];
            self.pointer_vec[index_a] = self.pointer_vec[index_b];
            self.heap_vec[index_b] = temp1;
            self.pointer_vec[index_b] = temp2;
        }

        fn sift_up(&mut self, index: usize) {
            let mut index = index;
            while index > 0 {
                if self.heap_vec[parent(index)] > self.heap_vec[index] {
                    self.swap(parent(index), index);
                    index = parent(index);
                } else {
                    break;
                }
            }
        }

        fn sift_down(&mut self, index: usize) {
            let mut index = index;
            while right_child(index) < self.heap_vec.len() {
                if self.heap_vec[left_child(index)] < self.heap_vec[index] {
                    self.swap(left_child(index), index);
                    index = left_child(index);
                } else if self.heap_vec[right_child(index)] < self.heap_vec[index] {
                    self.swap(right_child(index), index);
                    index = right_child(index);
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test_minheap {
    use super::*;

    #[test]
    fn test_push() {
        let mut heap = minheap::initialize();

        heap.push(12, 3);
        heap.push(4, 100);
        heap.push(5, 101);
        heap.push(1, 30);

        assert_eq!(heap.pop(), (1, 30));
        assert_eq!(heap.pop(), (4, 100));
        assert_eq!(heap.pop(), (5, 101));
        assert_eq!(heap.pop(), (12, 3));
    }
}
