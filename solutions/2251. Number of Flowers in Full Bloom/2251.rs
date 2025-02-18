// Time complexity: O((n+m)logn)
// Space complexity: O(n+m)
use std::cmp::Ordering;

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut starts: Vec<i32> = flowers.iter().map(|f| f[0]).collect();
        let mut ends: Vec<i32> = flowers.iter().map(|f| f[1]).collect();

        starts.sort_unstable();
        ends.sort_unstable();

        people
            .iter()
            .map(|&p| {
                let started = starts
                    .binary_search_by(|&s| {
                        if s > p {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    })
                    .unwrap_or_else(|x| x);
                let ended = ends
                    .binary_search_by(|&e| {
                        if e >= p {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    })
                    .unwrap_or_else(|x| x);
                (started - ended) as i32
            })
            .collect()
    }
}
