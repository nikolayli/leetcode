// Time complexity: O(klogn)
// Space complexity: O(1)
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut ans: i64 = 1;
        let mut i = 1;

        while i < k {
            let gap = Self::get_gap(ans, ans + 1, n as i64);
            if i + gap <= k {
                i += gap;
                ans += 1;
            } else {
                i += 1;
                ans *= 10;
            }
        }

        ans as i32
    }

    fn get_gap(mut a: i64, mut b: i64, n: i64) -> i32 {
        let mut gap = 0;

        while a <= n {
            gap += (n + 1).min(b) - a;
            a *= 10;
            b *= 10;
        }

        gap as i32
    }
}
