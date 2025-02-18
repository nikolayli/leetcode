// Time complexity: O(nlogk)
// Space complexity: O(k)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    min_heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth_largest = KthLargest {
            k: k as usize,
            min_heap: BinaryHeap::with_capacity(k as usize),
        };
        for num in nums {
            kth_largest.add(num);
        }
        kth_largest
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heapify(val);
        self.min_heap.peek().unwrap().0
    }

    fn heapify(&mut self, val: i32) {
        self.min_heap.push(Reverse(val));
        if self.min_heap.len() > self.k {
            self.min_heap.pop();
        }
    }
}
