// Time complexity: O(n^2)
// Space complexity: O(n)
impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let k = k as usize;
        let mut copy = word.clone();
        let mut count = 0;
        let _mod = "*".repeat(k);
        loop {
            copy = copy.chars().skip(k).collect::<String>() + &_mod;
            count += 1;
            if Solution::check(&word, &copy) {
                break;
            }
        }

        count
    }

    fn check(word: &str, copy: &str) -> bool {
        word.chars()
            .zip(copy.chars())
            .all(|(w, c)| c == '*' || w == c)
    }
}
