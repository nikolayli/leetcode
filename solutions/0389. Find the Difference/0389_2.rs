// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut count = [0; 26];

        for c in s.chars() {
            count[c as usize - b'a' as usize] += 1;
        }

        for c in t.chars() {
            if count[c as usize - b'a' as usize] == 0 {
                return c;
            }
            count[c as usize - b'a' as usize] -= 1;
        }

        panic!("No difference found");
    }
}
