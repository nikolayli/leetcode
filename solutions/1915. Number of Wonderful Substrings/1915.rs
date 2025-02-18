// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut ans = 0;
        let mut prefix = 0;
        let mut count: [i64; 1024] = [0; 1024];
        count[0] = 1;

        for c in word.chars() {
            prefix ^= 1 << (c as u8 - b'a');
            ans += count[prefix];
            for i in 0..10 {
                ans += count[prefix ^ (1 << i)];
            }
            count[prefix] += 1;
        }

        ans
    }
}
