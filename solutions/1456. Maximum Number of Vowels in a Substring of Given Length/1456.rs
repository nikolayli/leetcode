// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut ans = 0;
        let mut mx = 0;
        let k_vowels = "aeiou";
        let chars: Vec<char> = s.chars().collect();

        for i in 0..chars.len() {
            if k_vowels.contains(chars[i]) {
                mx += 1;
            }
            if i >= k && k_vowels.contains(chars[i - k]) {
                mx -= 1;
            }
            ans = ans.max(mx);
        }

        ans
    }
}
