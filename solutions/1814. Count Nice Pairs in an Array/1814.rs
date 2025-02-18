// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        const k_mod: i64 = 1_000_000_007;
        let mut ans = 0;
        let mut count = HashMap::new();

        for &num in &nums {
            *count.entry(num - Self::rev(num)).or_insert(0) += 1;
        }

        for freq in count.values() {
            ans = (ans + (freq * (freq - 1) / 2) % k_mod) % k_mod;
        }

        ans as i32
    }

    fn rev(mut n: i32) -> i32 {
        let mut x = 0;
        while n != 0 {
            x = x * 10 + n % 10;
            n /= 10;
        }

        x
    }
}
