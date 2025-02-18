// Time complexity: O(row⋅col⋅log(row⋅col))
// Space complexity: O(row⋅col)O(row⋅col)
use std::collections::VecDeque;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut l = 1;
        let mut r = cells.len() - 1;

        while l <= r {
            let m = l + (r - l) / 2;
            if Self::can_walk(m, row, col, &cells) {
                ans = m;
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        ans as i32
    }

    fn can_walk(day: usize, row: i32, col: i32, cells: &Vec<Vec<i32>>) -> bool {
        let mut matrix = vec![vec![0; col as usize]; row as usize];
        for i in 0..day {
            let x = cells[i][0] - 1;
            let y = cells[i][1] - 1;
            matrix[x as usize][y as usize] = 1;
        }

        let mut queue = VecDeque::new();

        for j in 0..col {
            if matrix[0][j as usize] == 0 {
                queue.push_back((0, j));
                matrix[0][j as usize] = 1;
            }
        }

        while let Some((i, j)) = queue.pop_front() {
            for k in 0..4 {
                let x = i + Self::dirs()[k];
                let y = j + Self::dirs()[k + 1];
                if x < 0 || x == row || y < 0 || y == col {
                    continue;
                }
                if matrix[x as usize][y as usize] == 1 {
                    continue;
                }
                if x == row - 1 {
                    return true;
                }
                queue.push_back((x, y));
                matrix[x as usize][y as usize] = 1;
            }
        }

        false
    }

    fn dirs() -> [i32; 5] {
        [0, 1, 0, -1, 0]
    }
}
