// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let mut bob_score = 0;
        for &num in possible.iter() {
            bob_score += if num != 0 { 1 } else { -1 };
        }

        let mut daniel_score = 0;
        for i in 0..possible.len() - 1 {
            daniel_score += if possible[i] != 0 { 1 } else { -1 };
            if daniel_score > bob_score - daniel_score {
                return (i + 1) as i32;
            }
        }

        -1
    }
}
