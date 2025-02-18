// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
        let mut ans = 0;

        for &battery_percentage in battery_percentages.iter() {
            if battery_percentage - ans > 0 {
                ans += 1;
            }
        }

        ans
    }
}
