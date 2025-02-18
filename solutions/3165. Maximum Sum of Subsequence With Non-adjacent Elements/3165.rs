// Time complexity: O(n + qsqrt(n))
// Space complexity: O(sqrt(n))
use std::cmp;
use std::mem;

impl Solution {
    pub fn maximum_sum_subsequence(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let part_size = cmp::max(8, (nums.len() as f64).sqrt() as usize);
        let mut parts: Vec<[i32; 4]> = Vec::new();

        for i in (0..nums.len()).step_by(part_size) {
            parts.push(Self::calculate_max_non_adjacent_sum(
                &nums[i..cmp::min(i + part_size, nums.len())],
                0,
                part_size,
            ));
        }

        for query in queries {
            let part_index = query[0] as usize / part_size;
            nums[query[0] as usize] = query[1];
            parts[part_index] = Self::calculate_max_non_adjacent_sum(
                &nums[part_index * part_size..cmp::min((part_index + 1) * part_size, nums.len())],
                0,
                part_size,
            );
            ans += Self::combine_parts(&parts) as i64;
            ans %= Self::MOD;
        }

        ans as i32
    }

    const MOD: i64 = 1_000_000_007;

    fn calculate_max_non_adjacent_sum(nums: &[i32], start: usize, end: usize) -> [i32; 4] {
        let mut first_last = 0;
        let mut first_second_last = 0;
        let mut second_last = 0;
        let mut second_second_last = 0;

        for i in start..cmp::min(nums.len(), end) {
            first_last = cmp::max(
                mem::replace(&mut first_second_last, first_last) + nums[i],
                first_last,
            );
            if i > start {
                second_last = cmp::max(
                    mem::replace(&mut second_second_last, second_last) + nums[i],
                    second_last,
                );
            }
        }

        [
            first_last,
            first_second_last,
            second_last,
            second_second_last,
        ]
    }

    fn combine_parts(parts: &[[i32; 4]]) -> i32 {
        let mut max_last = 0;
        let mut max_second_last = 0;

        for &[first_last, first_second_last, second_last, second_second_last] in parts {
            let temp = max_last;
            max_last = cmp::max(max_last + second_last, max_second_last + first_last);
            max_second_last = cmp::max(
                temp + second_second_last,
                max_second_last + first_second_last,
            );
        }

        max_last
    }
}
