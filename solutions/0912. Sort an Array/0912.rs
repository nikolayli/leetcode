// Time Complexity: O(nlogn)
// Space Complexity: O(n)
impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted = nums.clone();
        let len = sorted.len();
        Self::merge_sort(&mut sorted, 0, len - 1);

        sorted
    }

    fn merge_sort(nums: &mut Vec<i32>, l: usize, r: usize) {
        if l >= r {
            return;
        }

        let m = l + (r - l) / 2;
        Self::merge_sort(nums, l, m);
        Self::merge_sort(nums, m + 1, r);
        Self::merge(nums, l, m, r);
    }

    fn merge(nums: &mut Vec<i32>, l: usize, m: usize, r: usize) {
        let mut sorted = vec![0; r - l + 1];
        let mut k = 0;
        let mut i = l;
        let mut j = m + 1;

        while i <= m && j <= r {
            if nums[i] < nums[j] {
                sorted[k] = nums[i];
                i += 1;
            } else {
                sorted[k] = nums[j];
                j += 1;
            }
            k += 1;
        }

        while i <= m {
            sorted[k] = nums[i];
            i += 1;
            k += 1;
        }

        while j <= r {
            sorted[k] = nums[j];
            j += 1;
            k += 1;
        }

        for idx in 0..sorted.len() {
            nums[l + idx] = sorted[idx];
        }
    }
}
