// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        const K_MAX: usize = 100;
        let mut count = vec![0; K_MAX + 1];

        nums.iter().for_each(|&num| count[num as usize] += 1);

        (1..=K_MAX).for_each(|i| count[i] += count[i - 1]);

        nums.iter()
            .map(|&num| if num == 0 { 0 } else { count[num as usize - 1] })
            .collect()
    }
}
