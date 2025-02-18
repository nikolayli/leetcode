// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map_s = vec![0; 128];
        let mut map_t = vec![0; 128];

        for (i, (char_s, char_t)) in s.chars().zip(t.chars()).enumerate() {
            if map_s[char_s as usize] != map_t[char_t as usize] {
                return false;
            }
            map_s[char_s as usize] = i + 1;
            map_t[char_t as usize] = i + 1;
        }

        true
    }
}
