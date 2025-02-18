// Time complexity: O(nlogn)
// Space complexity: O(n)
impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let left = Self::length_of_lis(&nums);
        let right = Self::length_of_lis(&nums.iter().rev().cloned().collect::<Vec<_>>())
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<_>>();
        let mut max_mountain_seq = 0;

        for (l, r) in left.iter().zip(right.iter()) {
            if *l > 1 && *r > 1 {
                max_mountain_seq = (l + r - 1).max(max_mountain_seq)
            }
        }

        (nums.len() as i32) - max_mountain_seq
    }

    fn length_of_lis(nums: &[i32]) -> Vec<i32> {
        let mut tails = Vec::new();
        let mut dp = Vec::new();

        for &num in nums {
            if tails.is_empty() || num > *tails.last().unwrap() {
                tails.push(num);
            } else {
                let pos = tails.binary_search(&num).unwrap_or_else(|e| e);
                tails[pos] = num;
            }
            dp.push(tails.len() as i32);
        }

        dp
    }
}
