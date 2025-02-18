// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let k_vowels = "AEIOUaeiou";

        let mut count = vec![0; 128];
        for c in s.chars() {
            if k_vowels.contains(c) {
                count[c as usize] += 1;
            }
        }

        let mut ans = String::new();
        let mut j = 0;
        for (i, c) in s.chars().enumerate() {
            if !k_vowels.contains(c) {
                ans.push(c);
            } else {
                while count[k_vowels.chars().nth(j).unwrap() as usize] == 0 {
                    j += 1;
                }

                ans.push(k_vowels.chars().nth(j).unwrap());
                count[k_vowels.chars().nth(j).unwrap() as usize] -= 1;
            }
        }

        ans
    }
}
