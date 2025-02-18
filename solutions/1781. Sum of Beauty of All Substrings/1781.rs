// Time complexity: O(n^2)
// Space complexity: O(1)
impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let mut ans = 0;

        for i in 0..s.len() {
            let mut count = vec![0; 26];
            for j in i..s.len() {
                count[(s.as_bytes()[j] - b'a') as usize] += 1;
                ans +=
                    count.iter().max().unwrap() - count.iter().filter(|&&x| x > 0).min().unwrap();
            }
        }

        ans
    }
}
