// Time complexity: O(sort + nlog(max-min))
// Space complexity: O(sort)
impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();

        let mut l = 1;
        let mut r = position[position.len() - 1] - position[0];

        let num_balls = |force: i32| -> i32 {
            position
                .iter()
                .scan(-force, |prev_position, &pos| {
                    if pos - *prev_position >= force {
                        *prev_position = pos;
                        Some(1)
                    } else {
                        Some(0)
                    }
                })
                .sum()
        };

        while l < r {
            let mid = r - (r - l) / 2;
            if num_balls(mid) >= m {
                l = mid;
            } else {
                r = mid - 1;
            }
        }

        l
    }
}
