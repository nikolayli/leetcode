// Time complexity: O(sort)
// Space complexity: O(n)
impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity((nums.len() + 2) / 3);
        nums.sort_unstable();
        for chunk in nums.chunks(3) {
            if chunk[2] - chunk[0] > k {
                return Vec::new();
            }
            ans.push(chunk.to_vec());
        }

        ans
    }
}
