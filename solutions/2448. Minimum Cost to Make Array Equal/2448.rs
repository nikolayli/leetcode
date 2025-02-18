// Time complexity: O(nlog(max(nums)−min(nums)))
// Space complexity: O(1)
impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut l = *nums.iter().min().unwrap() as i64;
        let mut r = *nums.iter().max().unwrap() as i64;

        fn get_cost(nums: &Vec<i32>, cost: &Vec<i32>, target: i64) -> i64 {
            nums.iter()
                .zip(cost.iter())
                .map(|(&num, &c)| (num as i64 - target).abs() * c as i64)
                .sum()
        }

        while l < r {
            let m = (l + r) / 2;
            let cost1 = get_cost(&nums, &cost, m);
            let cost2 = get_cost(&nums, &cost, m + 1);
            if cost1 < cost2 {
                r = m;
            } else {
                l = m + 1;
            }
        }

        get_cost(&nums, &cost, l)
    }
}
