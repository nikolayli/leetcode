// Time complexity: O(mn)
// Space complexity: O(1)
impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut prev_ones = 0;

        for row in bank {
            let ones = row.chars().filter(|&c| c == '1').count() as i32;
            if ones > 0 {
                ans += prev_ones * ones;
                prev_ones = ones;
            }
        }

        ans
    }
}
