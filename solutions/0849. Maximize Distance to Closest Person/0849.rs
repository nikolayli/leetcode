// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let n = seats.len();
        let mut ans = 0;
        let mut j: Option<usize> = None;

        for i in 0..n {
            if seats[i] == 1 {
                ans = match j {
                    None => i,
                    Some(j_val) => ans.max((i - j_val) / 2),
                };
                j = Some(i);
            }
        }

        match j {
            None => ans as i32,
            Some(j_val) => ans.max(n - 1 - j_val) as i32,
        }
    }
}
