// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut i = 0;

        for (j, t) in typed.chars().enumerate() {
            if i < name.len() && name.chars().nth(i) == Some(t) {
                i += 1;
            } else if j == 0 || t != typed.chars().nth(j - 1).unwrap() {
                return false;
            }
        }

        i == name.len()
    }
}
