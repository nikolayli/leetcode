// Time Complexity: O(n * log(1 / precision))
// Space Complexity: O(1)
impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = arr.len();
        let mut l = 0.0;
        let mut r = 1.0;

        while l < r {
            let m = l + (r - l) / 2.0;
            let mut fractions_no_greater_than_m = 0;
            let mut p = 0;
            let mut q = 1;

            for i in 0..n {
                let mut j = 1;
                while j < n && arr[i] as f64 > m * arr[j] as f64 {
                    j += 1;
                }
                if j == n {
                    break;
                }
                fractions_no_greater_than_m += n - j;
                if p * arr[j] < q * arr[i] {
                    p = arr[i];
                    q = arr[j];
                }
            }

            if fractions_no_greater_than_m == k {
                return vec![p, q];
            }
            if fractions_no_greater_than_m > k {
                r = m;
            } else {
                l = m;
            }
        }

        panic!("No solution found.");
    }
}
