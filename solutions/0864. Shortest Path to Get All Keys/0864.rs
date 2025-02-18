// Time Complexity: O(mn2^k)
// Space Complexity: O(mn2^k)
use std::collections::VecDeque;

struct T {
    i: usize,
    j: usize,
    keys: usize,
}

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let m = grid.len();
        let n = grid[0].len();
        let keys_count = Self::get_keys_count(&grid);
        let k_keys = (1 << keys_count) - 1;
        let start = Self::get_start(&grid);
        let mut q = VecDeque::new();
        q.push_back(T {
            i: start[0],
            j: start[1],
            keys: 0,
        });
        let mut seen = vec![vec![vec![false; k_keys + 1]; n]; m];
        seen[start[0]][start[1]][0] = true;

        let mut step = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let T { i, j, keys } = q.pop_front().unwrap();
                for &(dx, dy) in &DIRS {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;
                    if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                        continue;
                    }
                    let x = x as usize;
                    let y = y as usize;
                    let c = grid[x].as_bytes()[y] as char;
                    if c == '#' {
                        continue;
                    }
                    let new_keys = if ('a'..='f').contains(&c) {
                        keys | 1 << (c as usize - 'a' as usize)
                    } else {
                        keys
                    };
                    if new_keys == k_keys {
                        return step + 1;
                    }
                    if seen[x][y][new_keys] {
                        continue;
                    }
                    if ('A'..='F').contains(&c)
                        && (new_keys >> (c as usize - 'A' as usize) & 1) == 0
                    {
                        continue;
                    }
                    q.push_back(T {
                        i: x,
                        j: y,
                        keys: new_keys,
                    });
                    seen[x][y][new_keys] = true;
                }
            }
            step += 1;
        }

        -1
    }

    fn get_keys_count(grid: &[String]) -> usize {
        grid.iter()
            .flat_map(|s| s.chars())
            .filter(|&c| ('a'..='f').contains(&c))
            .count()
    }

    fn get_start(grid: &[String]) -> Vec<usize> {
        for (i, row) in grid.iter().enumerate() {
            if let Some(j) = row.chars().position(|c| c == '@') {
                return vec![i, j];
            }
        }
        panic!("Start position not found");
    }
}
