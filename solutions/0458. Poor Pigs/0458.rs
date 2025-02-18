// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        (buckets as f32)
            .log((minutes_to_test / minutes_to_die + 1) as f32)
            .ceil() as i32
    }
}
