// Time complexity: O(n+m)
// Space complexity: O(n+m)
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len();
        let m = *score.iter().max().unwrap();

        let mut score_to_index = vec![0; (m + 1) as usize];
        for i in 0..n {
            score_to_index[score[i] as usize] = (i + 1) as i32;
        }

        let medals = vec!["Gold Medal", "Silver Medal", "Bronze Medal"];

        let mut ans = vec!["".to_string(); n];
        let mut place = 1;
        for i in (0..=m).rev() {
            if score_to_index[i as usize] != 0 {
                let original_index = score_to_index[i as usize] - 1;
                if place < 4 {
                    ans[original_index as usize] = medals[place as usize - 1].to_string();
                } else {
                    ans[original_index as usize] = place.to_string();
                }
                place += 1;
            }
        }

        ans
    }
}
