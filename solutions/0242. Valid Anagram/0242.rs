// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count: Vec<i32> = vec![0; 26];

        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        for c in t.chars() {
            let index = (c as u8 - b'a') as usize;
            if count[index] == 0 {
                return false;
            }
            count[index] -= 1;
        }

        true
    }
}
