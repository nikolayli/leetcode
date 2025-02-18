// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut ans = 0;
        let mut count = vec![0; 26];

        for &c in word.as_bytes() {
            count[(c - b'a') as usize] += 1;
        }

        count.sort_unstable_by_key(|&x| -x);

        for i in 0..26 {
            ans += count[i] * ((i as i32 / 8) + 1);
        }

        ans
    }
}
