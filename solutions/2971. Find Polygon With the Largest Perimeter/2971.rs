// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();
        let mut previous_elements_sum: i64 = 0;
        let mut ans: i64 = -1;

        for num in nums {
            let temp = num as i64;
            if temp < previous_elements_sum {
                ans = temp + previous_elements_sum;
            }
            previous_elements_sum += temp;
        }

        ans
    }
}
