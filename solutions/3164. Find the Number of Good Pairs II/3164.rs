// Time complexity: O(n*sqrt(m))
// Space complexity: O(n*sqrt(m))
use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;

        for &num in &nums1 {
            for i in 1..=((num as f64).sqrt() as i32) {
                if num % i == 0 {
                    *count.entry(i).or_insert(0) += 1;
                    if i * i != num {
                        *count.entry(num / i).or_insert(0) += 1;
                    }
                }
            }
        }

        for &num in &nums2 {
            let num = num * k;
            if let Some(&freq) = count.get(&num) {
                ans += freq as i64;
            }
        }

        ans
    }
}
