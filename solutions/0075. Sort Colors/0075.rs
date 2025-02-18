// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut count = [0_usize; 3];

        for num in nums.iter() {
            count[*num as usize] += 1;
        }

        count
            .into_iter()
            .enumerate()
            .flat_map(|(color, freq)| (0..freq).map(move |_| color as i32))
            .zip(nums.iter_mut())
            .for_each(|(color, num)| {
                *num = color;
            });
    }
}
