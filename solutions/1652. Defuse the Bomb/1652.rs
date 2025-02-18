// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut ans = vec![0; n];

        if k == 0 {
            return ans;
        }

        let mut sum = 0;
        let (mut start, mut end) = if k > 0 {
            (1, k as usize)
        } else {
            ((n as isize + k as isize) as usize, n - 1)
        };

        for i in start..=end {
            sum += code[i % n];
        }

        for i in 0..n {
            ans[i] = sum;
            sum -= code[start % n];
            start += 1;
            end += 1;
            sum += code[end % n];
        }

        ans
    }
}
