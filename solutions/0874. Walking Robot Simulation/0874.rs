// Time Complexity: O(m+n)
// Space Complexity: O(n)
use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut ans = 0;
        let mut x = 0;
        let mut y = 0;
        let mut d = 0;

        let mut obstacle_set: HashSet<(i32, i32)> = HashSet::new();
        for obs in obstacles {
            obstacle_set.insert((obs[0], obs[1]));
        }

        for command in commands {
            match command {
                -2 => d = (d + 3) % 4,
                -1 => d = (d + 1) % 4,
                k if k > 0 => {
                    for _ in 0..k {
                        let next_x = x + dirs[d].0;
                        let next_y = y + dirs[d].1;
                        if !obstacle_set.contains(&(next_x, next_y)) {
                            x = next_x;
                            y = next_y;
                            ans = ans.max(x * x + y * y);
                        } else {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        ans
    }
}
