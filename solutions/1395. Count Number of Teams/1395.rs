// Time complexity: O(n^2)
// Space complexity: O(1)
impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        (1..rating.len() - 1).fold(0, |ans, i| {
            let (left_less, left_greater) = (0..i).fold((0, 0), |(ll, lg), j| {
                if rating[j] < rating[i] {
                    (ll + 1, lg)
                } else if rating[j] > rating[i] {
                    (ll, lg + 1)
                } else {
                    (ll, lg)
                }
            });

            let (right_less, right_greater) = (i + 1..rating.len()).fold((0, 0), |(rl, rg), j| {
                if rating[j] < rating[i] {
                    (rl + 1, rg)
                } else if rating[j] > rating[i] {
                    (rl, rg + 1)
                } else {
                    (rl, rg)
                }
            });

            ans + left_less * right_greater + left_greater * right_less
        })
    }
}
