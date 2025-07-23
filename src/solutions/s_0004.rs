struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let nums1_slice = &nums1[..];
        let nums2_slice = &nums2[..];

        if nums1_slice.len() > nums2_slice.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let m = nums1_slice.len();
        let n = nums2_slice.len();
        let total_left = (m + n).div_ceil(2);

        let (mut left, mut right) = (0, m);
        while left <= right {
            let part1 = (left + right) / 2;
            let part2 = total_left - part1;

            let max_left1 = if part1 == 0 {
                i32::MIN
            } else {
                nums1_slice[part1 - 1]
            };
            let min_right1 = if part1 == m {
                i32::MAX
            } else {
                nums1_slice[part1]
            };
            let max_left2 = if part2 == 0 {
                i32::MIN
            } else {
                nums2_slice[part2 - 1]
            };
            let min_right2 = if part2 == n {
                i32::MAX
            } else {
                nums2_slice[part2]
            };

            if max_left1 <= min_right2 && max_left2 <= min_right1 {
                if (m + n) % 2 == 1 {
                    return f64::from(max_left1.max(max_left2));
                } else {
                    return f64::from(max_left1.max(max_left2) + min_right1.min(min_right2)) / 2.0;
                }
            } else if max_left1 > min_right2 {
                right = part1 - 1;
            } else {
                left = part1 + 1;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! find_median_sorted_arrays_test {
        ($name:ident: $nums1:expr, $nums2:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::find_median_sorted_arrays($nums1.to_vec(), $nums2.to_vec()),
                    $expected
                );
            }
        };
    }

    find_median_sorted_arrays_test!(case1: [1, 3], [2] => 2.0);
    find_median_sorted_arrays_test!(case2: [1, 2], [3, 4] => 2.5);
}
