// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::cmp;

impl Solution {
    pub fn maximize_square_hole_area(n: i32, m: i32, h_bars: Vec<i32>, v_bars: Vec<i32>) -> i32 {
        let gap = cmp::min(
            Self::max_continuous_gap(&h_bars),
            Self::max_continuous_gap(&v_bars),
        );
        gap * gap
    }

    fn max_continuous_gap(bars: &Vec<i32>) -> i32 {
        let mut res = 2;
        let mut running_gap = 2;
        let mut sorted_bars = bars.clone();
        sorted_bars.sort();
        for i in 1..sorted_bars.len() {
            running_gap = if sorted_bars[i] == sorted_bars[i - 1] + 1 {
                running_gap + 1
            } else {
                2
            };
            res = cmp::max(res, running_gap);
        }

        res
    }
}
