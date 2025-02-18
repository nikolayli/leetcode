// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = nums.len() + 1;
        let mut bits = vec![0; 32];

        let mut l = 0;
        for r in 0..nums.len() {
            Self::update(&mut bits, nums[r], 1);

            while l <= r && Self::bits_to_num(&bits) >= k {
                ans = ans.min(r - l + 1);
                Self::update(&mut bits, nums[l], -1);
                l += 1;
            }
        }

        if ans != nums.len() + 1 {
            ans as i32
        } else {
            -1
        }
    }

    fn update(bits: &mut Vec<i32>, x: i32, change: i32) {
        for i in 0..32 {
            if (x >> i) & 1 != 0 {
                bits[i] += change;
            }
        }
    }

    fn bits_to_num(bits: &Vec<i32>) -> i32 {
        let mut res = 0;

        for i in 0..32 {
            if bits[i] != 0 {
                res |= 1 << i;
            }
        }

        res
    }
}
