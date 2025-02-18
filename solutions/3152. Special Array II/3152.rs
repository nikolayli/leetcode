// Time complexity: O(n+m)
// Space complexity: O(n+m)
impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut ans = Vec::new();
        let mut id = 0;
        let mut parity_ids = vec![id];

        for i in 1..nums.len() {
            if nums[i] % 2 == nums[i - 1] % 2 {
                id += 1;
            }
            parity_ids.push(id);
        }

        for query in queries {
            let from = query[0] as usize;
            let to = query[1] as usize;
            ans.push(parity_ids[from] == parity_ids[to]);
        }

        ans
    }
}
