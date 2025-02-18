// Time complexity: O(N)
// Space complexity: O(1)
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut current_altitude = 0;
        let mut highest_point = current_altitude;

        for altitude_gain in gain {
            current_altitude += altitude_gain;
            highest_point = highest_point.max(current_altitude);
        }

        highest_point
    }
}
