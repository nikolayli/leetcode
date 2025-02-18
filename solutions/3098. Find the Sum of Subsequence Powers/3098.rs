// Time complexity: O(n^2*k)
// Space complexity: O(nk)
use std::collections::HashMap;

impl Solution {
    pub fn sum_of_powers(mut nums: Vec<i32>, k: i32) -> i32 {
        const KMOD: i32 = 1_000_000_007;
        let mut memo = HashMap::new();

        nums.sort_unstable();

        fn dp(
            i: usize,
            k: i32,
            last_picked_index: isize,
            first_index: isize,
            second_index: isize,
            nums: &Vec<i32>,
            memo: &mut HashMap<(usize, i32, isize, isize, isize), i32>,
        ) -> i32 {
            if k == 0 {
                return nums[second_index as usize] - nums[first_index as usize];
            }
            if i == nums.len() {
                return 0;
            }
            if let Some(&result) = memo.get(&(i, k, last_picked_index, first_index, second_index)) {
                return result;
            }

            let mut new_first_index = first_index;
            let mut new_second_index = second_index;
            if first_index == -1 {
                new_first_index = i as isize;
            } else if second_index == -1 {
                new_second_index = i as isize;
            } else if nums[i] - nums[last_picked_index as usize]
                < nums[second_index as usize] - nums[first_index as usize]
            {
                new_first_index = last_picked_index;
                new_second_index = i as isize;
            }

            let pick = dp(
                i + 1,
                k - 1,
                i as isize,
                new_first_index,
                new_second_index,
                nums,
                memo,
            );
            let skip = dp(
                i + 1,
                k,
                last_picked_index,
                first_index,
                second_index,
                nums,
                memo,
            );
            let result = (pick + skip) % KMOD;

            memo.insert((i, k, last_picked_index, first_index, second_index), result);
            result
        }

        dp(0, k, -1, -1, -1, &nums, &mut memo)
    }
}
