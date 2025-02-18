// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>) -> i64 {
        let mut ans: i64 = 0;
        let mut stack: Vec<(i32, i64)> = Vec::new();

        for &num in &nums {
            while !stack.is_empty() && stack.last().unwrap().0 < num {
                stack.pop();
            }
            if stack.is_empty() || stack.last().unwrap().0 != num {
                stack.push((num, 0));
            }
            ans += stack.last_mut().unwrap().1 + 1;
            stack.last_mut().unwrap().1 += 1;
        }

        ans
    }
}
