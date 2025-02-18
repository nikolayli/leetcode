// Time complexity: O(n+m)
// Space complexity: O(1)
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s2 = s2.as_bytes();
        let mut count = vec![0; 26];
        let required = s1.len() as i32;

        for &c in s1.as_bytes() {
            count[(c - b'a') as usize] += 1;
        }

        for r in 0..s2.len() {
            let index = (s2[r] - b'a') as usize;
            count[index] -= 1;
            if count[index] >= 0 {
                required -= 1;
            }
            if r >= s1.len() {
                let index = (s2[r - s1.len()] - b'a') as usize;
                count[index] += 1;
                if count[index] > 0 {
                    required += 1;
                }
            }
            if required == 0 {
                return true;
            }
        }

        false
    }
}
