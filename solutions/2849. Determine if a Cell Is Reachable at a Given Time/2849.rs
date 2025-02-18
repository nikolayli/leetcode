// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let min_step = std::cmp::max((sx - fx).abs(), (sy - fy).abs());
        if min_step == 0 {
            return t != 1;
        }

        min_step <= t
    }
}
