// Time complexity: O(sort)
// Space complexity: O(n)
impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();
        let mut prev = String::new();

        folder.sort_unstable();

        for f in folder {
            if !prev.is_empty() && f.starts_with(&prev) && f.chars().nth(prev.len()) == Some('/') {
                continue;
            }
            ans.push(f.clone());
            prev = f;
        }

        ans
    }
}
