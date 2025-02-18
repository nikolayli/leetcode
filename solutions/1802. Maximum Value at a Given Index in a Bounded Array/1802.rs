// Time complexity: O(log(maxSum))
// Space complexity: O(1)
impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let mut max_sum = (max_sum - n);
        let (mut l, mut r) = (0, max_sum);

        while l < r {
            let m = l + (r - l) / 2;
            if Self::get_sum(n, index, m) >= max_sum as i64 {
                r = m;
            } else {
                l = m + 1;
            }
        }

        if Self::get_sum(n, index, l) > max_sum as i64 {
            l
        } else {
            l + 1
        }
    }

    fn get_sum(n: i32, index: i32, x: i32) -> i64 {
        let l = index.min(x - 1) as i64;
        let r = (n - index).min(x) as i64;
        let l_sum = ((x - 1) + (x - 1 - l as i32 + 1)) as i64 * l / 2;
        let r_sum = (x + (x - r as i32 + 1)) as i64 * r / 2;

        l_sum + r_sum
    }
}
