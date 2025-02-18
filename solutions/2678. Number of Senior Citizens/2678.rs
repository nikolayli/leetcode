// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .filter(|s| s[11..13].parse::<i32>().unwrap() > 60)
            .count() as i32
    }
}
