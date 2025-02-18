// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut count = num_bottles;

        while num_bottles >= num_exchange {
            num_bottles -= num_exchange - 1;
            num_exchange += 1;
            count += 1;
        }

        count
    }
}
