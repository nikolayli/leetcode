// Time complexity: O(m+nk)
// Space complexity: O(1)
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut is_allowed = vec![false; 26];
        for c in allowed.chars() {
            is_allowed[(c as u8 - b'a') as usize] = true;
        }

        let mut ans = 0;

        for word in words {
            let mut is_consistent = true;
            for c in word.chars() {
                if !is_allowed[(c as u8 - b'a') as usize] {
                    is_consistent = false;
                    break;
                }
            }
            if is_consistent {
                ans += 1;
            }
        }

        ans
    }
}
