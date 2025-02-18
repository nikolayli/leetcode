// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let (dp, count_b) = s.chars().fold((0, 0), |(mut dp, mut count_b), c| {
            if c == 'a' {
                dp = (dp + 1).min(count_b);
            } else {
                count_b += 1;
            }
            (dp, count_b)
        });
        dp as i32
    }
}
