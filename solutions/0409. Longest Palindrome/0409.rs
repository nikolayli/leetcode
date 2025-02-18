// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count = vec![0; 128];
        let mut ans = 0;

        for c in s.chars() {
            count[c as usize] += 1;
        }

        let mut has_odd_count = false;
        for &freq in &count {
            if freq % 2 == 0 {
                ans += freq;
            } else {
                ans += freq - 1;
                has_odd_count = true;
            }
        }

        if has_odd_count {
            ans += 1;
        }

        ans
    }
}
