// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut k = k as i64;
        let mut length = 0;

        for c in s.chars() {
            if c.is_ascii_digit() {
                length *= (c as i64) - ('0' as i64);
            } else {
                length += 1;
            }
        }

        for c in s.chars().rev() {
            k %= length;
            if k == 0 && c.is_ascii_alphabetic() {
                return c.to_string();
            }
            if c.is_ascii_digit() {
                length /= (c as i64) - ('0' as i64);
            } else {
                length -= 1;
            }
        }

        panic!("No solution found");
    }
}
