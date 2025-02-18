// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn compressed_string(word: String) -> String {
        let n = word.len();
        let mut ans = String::new();
        let mut chars = word.chars().peekable();

        while let Some(&current_char) = chars.peek() {
            let mut count = 0;
            while chars.peek() == Some(&current_char) && count < 9 {
                chars.next();
                count += 1;
            }
            ans.push_str(&count.to_string());
            ans.push(current_char);
        }

        ans
    }
}
