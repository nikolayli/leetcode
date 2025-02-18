// Time complexity: O(mn)
// Space complexity: O(1)
impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        fn is_magic(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
            let mut seen = [false; 10];
            let mut sums = [0; 8];

            for i in 0..3 {
                for j in 0..3 {
                    let num = grid[row + i][col + j];
                    if num < 1 || num > 9 || seen[num as usize] {
                        return false;
                    }
                    seen[num as usize] = true;

                    sums[i] += num;
                    sums[3 + j] += num;
                    if i == j {
                        sums[6] += num;
                    }
                    if i + j == 2 {
                        sums[7] += num;
                    }
                }
            }

            sums.iter().all(|&x| x == sums[0])
        }

        let m = grid.len();
        let n = grid[0].len();
        if m < 3 || n < 3 {
            return 0;
        }

        let mut ans = 0;
        for i in 0..m - 2 {
            for j in 0..n - 2 {
                if is_magic(&grid, i, j) {
                    ans += 1;
                }
            }
        }

        ans
    }
}
