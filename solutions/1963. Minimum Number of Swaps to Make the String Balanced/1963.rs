// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut unmatched = 0;

        for c in s.chars() {
            match c {
                '[' => unmatched += 1,
                ']' => {
                    if unmatched > 0 {
                        unmatched -= 1;
                    }
                }
                _ => unreachable!(),
            }
        }

        (unmatched + 1) / 2
    }
}
