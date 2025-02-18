// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn is_valid(word: String) -> bool {
        const k_vowels: &str = "aeiouAEIOU";

        fn is_consonant(c: char) -> bool {
            c.is_alphabetic() && !k_vowels.contains(c)
        }

        word.len() >= 3
            && word.chars().all(|c| c.is_alphanumeric())
            && word.chars().any(|c| k_vowels.contains(c))
            && word.chars().any(is_consonant)
    }
}
