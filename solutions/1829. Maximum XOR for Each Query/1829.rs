// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mx = (1 << maximum_bit) - 1;
        let mut ans = Vec::new();
        let mut xors = 0;

        for &num in &nums {
            xors ^= num;
            ans.push(xors ^ mx);
        }

        ans.reverse();
        ans
    }
}
