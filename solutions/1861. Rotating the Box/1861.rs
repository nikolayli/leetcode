// Time complexity: O(mn)
// Space complexity: O(mn)
impl Solution {
    pub fn rotate_the_box(r#box: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = r#box.len();
        let n = r#box[0].len();
        let mut ans = vec![vec!['.'; m]; n];

        for i in 0..m {
            let mut k = n - 1;
            for j in (0..n).rev() {
                if r#box[i][j] != '.' {
                    if r#box[i][j] == '*' {
                        k = j;
                    }
                    ans[k][m - i - 1] = r#box[i][j];
                    if k > 0 {
                        k -= 1;
                    }
                }
            }
        }

        ans
    }
}
