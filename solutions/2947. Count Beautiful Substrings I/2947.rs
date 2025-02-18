// Time complexity: O(n^2)
// Space complexity: O(1)
impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        let mut ans = 0;
        let is_vowel = vec!['a', 'e', 'i', 'o', 'u'];

        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        for i in 0..n {
            let mut vowels = 0;
            let mut consonants = 0;
            for j in i..n {
                if is_vowel.contains(&s[j]) {
                    vowels += 1;
                } else {
                    consonants += 1;
                }

                if vowels == consonants && (vowels * consonants) % k == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}
