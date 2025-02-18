// Time complexity: O(max(m,n))
// Space complexity: O(m+n)
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let n = word1.len().min(word2.len());
        let mut ans = String::with_capacity(word1.len() + word2.len());

        for i in 0..n {
            ans.push(word1.chars().nth(i).unwrap());
            ans.push(word2.chars().nth(i).unwrap());
        }

        ans.push_str(&word1[n..]);
        ans.push_str(&word2[n..]);

        ans
    }
}
