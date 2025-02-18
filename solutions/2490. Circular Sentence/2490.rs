// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let chars: Vec<char> = sentence.chars().collect();
        let n = chars.len();

        for i in 1..n {
            if chars[i] == ' ' && chars[i - 1] != chars[i + 1] {
                return false;
            }
        }

        chars[0] == chars[n - 1]
    }
}
