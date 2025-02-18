// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct T {
    pro: i32,
    cap: i32,
}

impl Ord for T {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cap.cmp(&other.cap)
    }
}

impl PartialOrd for T {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut min_heap = BinaryHeap::new();
        let mut max_heap = BinaryHeap::new();

        for i in 0..capital.len() {
            min_heap.push(Reverse(T {
                pro: profits[i],
                cap: capital[i],
            }));
        }

        for _ in 0..k {
            while let Some(Reverse(top)) = min_heap.peek() {
                if top.cap <= w {
                    max_heap.push(min_heap.pop().unwrap().0.pro);
                } else {
                    break;
                }
            }
            if let Some(profit) = max_heap.pop() {
                w += profit;
            } else {
                break;
            }
        }

        w
    }
}
