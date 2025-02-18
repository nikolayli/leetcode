// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut ans: Vec<bool> = vec![];
        let mut max_candies: i32 = 0;

        for &candy in &candies {
            max_candies = max_candies.max(candy);
        }

        for &candy in &candies {
            ans.push(candy + extra_candies >= max_candies);
        }

        ans
    }
}
