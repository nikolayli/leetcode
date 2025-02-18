// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let s = (nums.len() * (nums.len() + 1) / 2) as i32;
        let (mut a, mut u) = (0i32, 0i32);
        let mut fl = vec![1; 10001];
        nums.into_iter().for_each(|n| {
            a += n;
            u += n * fl[n as usize];
            fl[n as usize] = 0;
        });

        vec![a - u, s - u]
    }
}
