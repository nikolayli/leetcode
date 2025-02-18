// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut count = vec![0; 101];

        for num in nums {
            ans += count[num as usize];
            count[num as usize] += 1;
        }

        ans
    }
}
