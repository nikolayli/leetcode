// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        let mut ans = 0;
        let mut score = 0;
        let mut left = 0;
        let mut right = tokens.len();

        tokens.sort_unstable();

        while left < right {
            if power >= tokens[left] {
                power -= tokens[left];
                score += 1;
                left += 1;
            } else if score > 0 {
                power += tokens[right - 1];
                score -= 1;
                right -= 1;
            } else {
                break;
            }
            ans = ans.max(score);
        }

        ans
    }
}
