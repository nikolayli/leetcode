// Time Complexity: O(n)
// Space Complexity: O(n)
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut nums = vec![1];
        let (mut i2, mut i3, mut i5) = (0, 0, 0);

        while nums.len() < n {
            let next2 = nums[i2] * 2;
            let next3 = nums[i3] * 3;
            let next5 = nums[i5] * 5;
            let next = next2.min(next3).min(next5);
            if next == next2 {
                i2 += 1;
            }
            if next == next3 {
                i3 += 1;
            }
            if next == next5 {
                i5 += 1;
            }
            nums.push(next);
        }

        nums[n - 1]
    }
}
