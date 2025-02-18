// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let k = k as usize;
        let n = s.len();
        let mut ans = n as i32;
        let mut count = vec![0; 3];

        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        if count[0] < k || count[1] < k || count[2] < k {
            return -1;
        }

        let s = s.as_bytes();
        let mut l = 0;
        for r in 0..n {
            count[(s[r] - b'a') as usize] -= 1;
            while count[(s[r] - b'a') as usize] < k {
                count[(s[l] - b'a') as usize] += 1;
                l += 1;
            }
            ans = ans.min((n - (r - l + 1)) as i32);
        }

        ans
    }
}
