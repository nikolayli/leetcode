// Time complexity: O(⁡nlogn+nlogk)
// Space complexity: O(n+k)
use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut workers: Vec<(f64, i32)> = quality
            .iter()
            .zip(wage.iter())
            .map(|(&q, &w)| (w as f64 / q as f64, q))
            .collect();

        workers.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let mut max_heap = BinaryHeap::new();
        let mut quality_sum = 0;
        let mut ans = f64::MAX;

        for &(wage_per_quality, q) in &workers {
            max_heap.push(q);
            quality_sum += q;
            if max_heap.len() > k {
                if let Some(max_q) = max_heap.pop() {
                    quality_sum -= max_q;
                }
            }
            if max_heap.len() == k {
                ans = ans.min(quality_sum as f64 * wage_per_quality);
            }
        }

        ans
    }
}
