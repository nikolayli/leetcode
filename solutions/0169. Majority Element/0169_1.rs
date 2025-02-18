// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut count = 0;

        for num in nums {
            if count == 0 {
                ans = num;
            }
            count += match num == ans {
                true => 1,
                false => -1,
            };
        }

        ans
    }
}
