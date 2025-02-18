// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let mut count1 = [0; 26];
        let mut count2 = [0; 26];
        for c in word1.chars() {
            count1[c as usize - 0x61] += 1;
        }
        for c in word2.chars() {
            count2[c as usize - 0x61] += 1;
        }

        for i in 0..26 {
            if (count1[i] == 0 && count2[i] != 0) || (count1[i] != 0 && count2[i] == 0) {
                return false;
            }
        }

        count1.sort();
        count2.sort();

        count1 == count2
    }
}
