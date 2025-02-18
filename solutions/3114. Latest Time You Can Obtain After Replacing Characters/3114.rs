// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn find_latest_time(s: String) -> String {
        let mut s_cahrs: Vec<char> = s.chars().collect();

        if s_cahrs[0] == '?' {
            if s_cahrs[1] == '?' || s_cahrs[1] == '0' || s_cahrs[1] == '1' {
                s_cahrs[0] = '1';
            } else {
                s_cahrs[0] = '0';
            }
        }
        if s_cahrs[1] == '?' {
            if s_cahrs[0] == '1' {
                s_cahrs[1] = '1';
            } else {
                s_cahrs[1] = '9';
            }
        }
        if s_cahrs[3] == '?' {
            s_cahrs[3] = '5';
        }
        if s_cahrs[4] == '?' {
            s_cahrs[4] = '9';
        }

        s_cahrs.into_iter().collect()
    }
}
