// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut prev_set_bits = 0;
        let mut prev_max = i32::MIN;
        let mut curr_max = i32::MIN;
        let mut curr_min = i32::MAX;

        for &num in &nums {
            let set_bits = num.count_ones();
            if set_bits != prev_set_bits {
                if prev_max > curr_min {
                    return false;
                }
                prev_set_bits = set_bits;
                prev_max = curr_max;
                curr_max = num;
                curr_min = num;
            } else {
                curr_max = curr_max.max(num);
                curr_min = curr_min.min(num);
            }
        }

        prev_max <= curr_min
    }
}
