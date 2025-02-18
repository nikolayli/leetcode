// Time complexity: O((mn)!)
// Space complexity: O((mn)!)
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        const dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        const m: usize = 2;
        const n: usize = 3;
        const goal: &str = "123450";

        let mut start = String::new();
        for i in 0..m {
            for j in 0..n {
                start.push((board[i][j] as u8 + b'0') as char);
            }
        }

        if start == goal {
            return 0;
        }

        let mut q = VecDeque::new();
        let mut seen = HashSet::new();
        q.push_back((start.clone(), 0));
        seen.insert(start);

        while let Some((s, step)) = q.pop_front() {
            let zero_index = s.find('0').unwrap();
            let i = zero_index / n;
            let j = zero_index % n;

            for &(dx, dy) in &dirs {
                let x = i as i32 + dx;
                let y = j as i32 + dy;
                if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                    continue;
                }
                let swapped_index = (x as usize) * n + (y as usize);
                let mut new_s: Vec<char> = s.chars().collect();
                new_s.swap(zero_index, swapped_index);
                let new_s: String = new_s.into_iter().collect();
                if new_s == goal {
                    return step + 1;
                }
                if seen.insert(new_s.clone()) {
                    q.push_back((new_s, step + 1));
                }
            }
        }

        -1
    }
}
