// Time complexity: O(nlogn)
// Space complexity: O(n)
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(Vec::new(), |mut stk: Vec<i32>, n| {
                let i = stk.binary_search(&n).unwrap_or_else(|x| x);
                if i == stk.len() {
                    stk.push(n);
                } else {
                    stk[i] = n;
                }

                stk
            })
            .len() as i32
    }
}
