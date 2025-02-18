// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        const k_mod: i64 = 1_000_000_007;
        let mut ans = 1;

        for i in 1..=n {
            ans = ans * i as i64 * (i as i64 * 2 - 1) % k_mod;
        }

        ans as i32
    }
}
