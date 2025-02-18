// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut count = vec![0; 128];

        let mut l = 0;
        for r in 0..s.len() {
            count[s[r] as usize] += 1;

            while count[s[r] as usize] > 1 {
                count[s[l] as usize] -= 1;
                l += 1;
            }
            ans = ans.max(r - l + 1);
        }

        ans as i32
    }
}
