// Time complexity: O(nk)
// Space complexity: O(n)
impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let mut max_sum = vec![0; n + 1];

        for start in (0..n).rev() {
            let mut curr_max = 0;
            for i in start..n.min(start + k as usize) {
                curr_max = curr_max.max(arr[i]);
                max_sum[start] =
                    max_sum[start].max(max_sum[i + 1] + curr_max * (i - start + 1) as i32);
            }
        }

        max_sum[0]
    }
}
