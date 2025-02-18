// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut freq = vec![0; nums.len() + 1];
        let mut ans = Vec::new();

        for num in nums {
            if freq[num as usize] >= ans.len() {
                ans.push(Vec::new());
            }

            ans[freq[num as usize]].push(num);
            freq[num as usize] += 1;
        }

        ans
    }
}
