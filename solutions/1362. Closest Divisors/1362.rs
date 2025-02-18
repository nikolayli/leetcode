// Time complexity: O(sqrt(num))
// Space complexity: O(1)
impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let max_root = ((num + 2) as f64).sqrt() as i32;

        for root in (1..=max_root).rev() {
            for &cand in &[num + 1, num + 2] {
                if cand % root == 0 {
                    return vec![root, cand / root];
                }
            }
        }

        unreachable!()
    }
}
