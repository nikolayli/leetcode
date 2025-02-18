// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first: i32 = i32::MAX;
        let mut second: i32 = i32::MAX;

        for num in nums {
            if num <= first {
                first = num;
            } else if num <= second {
                second = num;
            } else {
                return true;
            }
        }
        false
    }
}
