// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut pos = 0;

        for num in nums {
            pos += num;
            if pos == 0 {
                count += 1;
            }
        }

        count
    }
}
