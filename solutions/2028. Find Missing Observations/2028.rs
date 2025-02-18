// Time complexity: O(m+n)
// Space complexity: O(n)
impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let target_sum = (rolls.len() as i32 + n) * mean;
        let current_sum: i32 = rolls.iter().sum();
        let missing_sum = target_sum - current_sum;
        if missing_sum > n * 6 || missing_sum < n {
            return vec![];
        }

        let mut ans = vec![missing_sum / n; n as usize];
        for i in 0..(missing_sum % n) as usize {
            ans[i] += 1;
        }

        ans
    }
}
