// Time complexity: O(mn^2)
// Space complexity: O(mn)
impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let start = start as usize;
        let finish = finish as usize;
        let fuel = fuel as usize;

        const MOD: i32 = 1_000_000_007;
        let n = locations.len();
        let mut dp = vec![vec![0; fuel + 1]; n];

        for f in 0..=fuel {
            dp[finish][f] = 1;
        }

        for f in 0..=fuel {
            for i in 0..n {
                for j in 0..n {
                    if i == j {
                        continue;
                    }
                    let required_fuel = (locations[i] - locations[j]).abs() as usize;
                    if required_fuel <= f {
                        dp[i][f] = (dp[i][f] + dp[j][f - required_fuel]) % MOD;
                    }
                }
            }
        }

        dp[start][fuel]
    }
}
