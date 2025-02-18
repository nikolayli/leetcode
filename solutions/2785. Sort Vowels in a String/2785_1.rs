// Time complexity: O(sort)
// Space complexity: O(n)
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let k_vowels = "aeiouAEIOU";
        let mut vowels: Vec<char> = s.chars().filter(|c| k_vowels.contains(*c)).collect();
        vowels.sort_unstable();

        let mut ans = Vec::new();
        let mut i = 0;
        for c in s.chars() {
            if k_vowels.contains(c) {
                ans.push(vowels[i]);
                i += 1;
            } else {
                ans.push(c);
            }
        }

        ans.into_iter().collect()
    }
}
