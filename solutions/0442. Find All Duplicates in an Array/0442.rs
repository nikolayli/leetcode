// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        for i in 0..nums.len() {
            let index = (nums[i].unsigned_abs() - 1) as usize;
            if nums[index] < 0 {
                ans.push((index + 1) as i32)
            }
            nums[index] *= -1;
        }

        ans
    }
}
