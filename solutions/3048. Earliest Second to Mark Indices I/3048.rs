// Time complexity: O((m + n)log⁡n)
// Space complexity: O(n)
impl Solution {
    fn enableToMarkIndices(nums: &Vec<i32>, change_indices: &Vec<i32>, m: i32) -> bool {
        let n = nums.len();
        let mut last_indices: Vec<(i32, usize)> = Vec::new();
        let mut sum: i32 = 0;

        last_indices.reserve(n);
        for i in 0..n {
            last_indices.push((-1, i));
        }

        for i in 0..m {
            last_indices[change_indices[i as usize] as usize].0 = i as i32;
        }

        last_indices.sort();
        for (i, (last, x)) in last_indices.into_iter().enumerate() {
            sum += nums[x];
            if (last < (i as i32) + sum) {
                return false;
            }
        }

        return true;
    }

    pub fn earliest_second_to_mark_indices(nums: Vec<i32>, change_indices: Vec<i32>) -> i32 {
        let mut change_indices = change_indices;
        let m = change_indices.len() as i64;
        let n = nums.len() as i64;
        let mut low: i64;
        let mut high: i64 = m + 1;
        let mut mid: i64;

        for x in change_indices.iter_mut() {
            *x -= 1;
        }

        low = nums.iter().fold(0 as i64, |sum, x| sum + (*x as i64));
        high = m + 1;

        if (low > m) {
            return -1;
        }

        while (low < high) {
            mid = low + (high - low) / 2;

            if (Self::enableToMarkIndices(&nums, &change_indices, mid as i32)) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        return if (low <= m) { low as i32 } else { -1 };
    }
}
