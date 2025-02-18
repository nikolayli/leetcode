// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut mn = i32::MAX;
        let mut mx = i32::MIN;
        let mut flag = false;

        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                flag = true;
            }
            if flag {
                mn = mn.min(nums[i]);
            }
        }

        flag = false;

        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                flag = true;
            }
            if flag {
                mx = mx.max(nums[i]);
            }
        }

        let mut l = 0;
        while l < nums.len() && nums[l] <= mn {
            l += 1;
        }

        let mut r = nums.len() - 1;
        while r > 0 && nums[r] >= mx {
            r -= 1;
        }

        if l >= r {
            0
        } else {
            (r - l + 1) as i32
        }
    }
}
