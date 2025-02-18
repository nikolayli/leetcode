// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut ans = vec!['.'; s.len()];

        for (i, &index) in indices.iter().enumerate() {
            ans[index as usize] = s.chars().nth(i).unwrap();
        }

        ans.into_iter().collect()
    }
}
