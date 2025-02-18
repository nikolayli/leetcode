// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut counts = [0; 26];

        for c in s.bytes() {
            counts[(c - b'a') as usize] += 1;
        }

        for c in t.bytes() {
            counts[(c - b'a') as usize] -= 1;
        }

        counts.iter().map(|&count: &i32| count.abs()).sum::<i32>() >> 1
    }
}
