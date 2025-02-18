// Time complexity: O(n*min(n,m))
// Space complexity: O(n*min(n,m))
impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        const KMOD: i32 = 1_000_000_007;
        let max_pos = (steps / 2 + 1).min(arr_len) as usize;
        let mut dp = vec![0; max_pos];
        dp[0] = 1;

        for _ in 0..steps {
            let mut new_dp = vec![0; max_pos];
            for (i, &ways) in dp.iter().enumerate() {
                if ways > 0 {
                    for &dx in &[-1, 0, 1] {
                        let next_index = i as i32 + dx;
                        if next_index >= 0 && (next_index as usize) < max_pos {
                            new_dp[next_index as usize] =
                                (new_dp[next_index as usize] + ways) % KMOD;
                        }
                    }
                }
            }
            dp = new_dp;
        }

        dp[0]
    }
}
