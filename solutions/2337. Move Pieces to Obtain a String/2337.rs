// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let n = start.len();
        let start_chars: Vec<char> = start.chars().collect();
        let target_chars: Vec<char> = target.chars().collect();
        let mut i = 0;
        let mut j = 0;

        while i <= n && j <= n {
            while i < n && start_chars[i] == '_' {
                i += 1;
            }
            while j < n && target_chars[j] == '_' {
                j += 1;
            }
            if i == n || j == n {
                return i == n && j == n;
            }
            if start_chars[i] != target_chars[j] {
                return false;
            }
            if start_chars[i] == 'R' && i > j {
                return false;
            }
            if start_chars[i] == 'L' && i < j {
                return false;
            }
            i += 1;
            j += 1;
        }

        true
    }
}
