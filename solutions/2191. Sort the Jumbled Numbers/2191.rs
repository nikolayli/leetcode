// Time complexity: O(sort)
// Space complexity: O(n)
impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let get_mapped = |mut num: i32| -> i32 {
            if num == 0 {
                return mapping[0];
            }
            let mut mapped = 0;
            let mut multiplier = 1;

            while num > 0 {
                let digit = num % 10;
                mapped += mapping[digit as usize] * multiplier;
                multiplier *= 10;
                num /= 10;
            }

            mapped
        };

        let mut a: Vec<(i32, i32)> = nums.iter().map(|&num| (get_mapped(num), num)).collect();

        a.sort_unstable_by_key(|&(mapped, _)| mapped);

        let ans: Vec<i32> = a.iter().map(|&(_, num)| num).collect();

        ans
    }
}
