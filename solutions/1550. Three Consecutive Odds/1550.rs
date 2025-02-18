// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for &a in &arr {
            if a % 2 == 1 {
                count += 1
            } else {
                count = 0;
            }
            if count == 3 {
                return true;
            }
        }
        false
    }
}
