// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut count = [0; 26];

        let s_bytes = s.as_bytes();
        let mut l = 0;
        let mut r = 0;

        while r < s_bytes.len() {
            count[(s_bytes[r] - b'a') as usize] += 1;

            while count[(s_bytes[r] - b'a') as usize] > 2 {
                count[(s_bytes[l] - b'a') as usize] -= 1;
                l += 1;
            }

            max_len = max_len.max(r - l + 1);
        }

        max_len as i32
    }
}
