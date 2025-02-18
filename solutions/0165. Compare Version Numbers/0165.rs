// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1 = version1.split('.').map(|s| s.parse().unwrap());
        let mut v2 = version2.split('.').map(|s| s.parse().unwrap());

        loop {
            match (v1.next(), v2.next()) {
                (Some(v1), v2) if v1 > v2.unwrap_or(0) => return 1,
                (v1, Some(v2)) if v2 > v1.unwrap_or(0) => return -1,
                (None, None) => return 0,
                _ => continue,
            }
        }
    }
}
