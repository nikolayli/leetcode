// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let total_chalk: i64 = chalk.iter().map(|&x| x as i64).sum();
        let mut k = k as i64 % total_chalk;

        for (i, &c) in chalk.iter().enumerate() {
            if k < c as i64 {
                return i as i32;
            }
            k -= c as i64;
        }

        0
    }
}
