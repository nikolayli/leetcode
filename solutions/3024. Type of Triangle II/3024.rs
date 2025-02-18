// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        nums.sort();

        if nums[0] + nums[1] <= nums[2] {
            return "none".to_string();
        }
        if nums[0] == nums[1] && nums[1] == nums[2] {
            return "equilateral".to_string();
        }
        if nums[0] == nums[1] || nums[1] == nums[2] {
            return "isosceles".to_string();
        }
        "scalene".to_string()
    }
}
