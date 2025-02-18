// Time complexity : O(n)
// Space complexity : O(n)
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; temperatures.len()];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..temperatures.len() {
            while !stack.is_empty() && temperatures[*stack.last().unwrap()] < temperatures[i] {
                let index = stack.pop().unwrap();
                ans[index] = (i - index) as i32;
            }
            stack.push(i);
        }

        ans
    }
}
