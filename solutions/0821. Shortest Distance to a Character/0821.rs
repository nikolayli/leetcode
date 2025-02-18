// Time Complexity: O(n)
// Space Complexity: O(n)
impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let n = s.len();
        let mut ans = vec![0; n];
        let mut prev = -(n as i32);

        for (i, ch) in s.chars().enumerate() {
            if ch == c {
                prev = i as i32;
            }
            ans[i] = (i as i32) - prev;
        }

        prev = 2 * (n as i32);
        let chars: Vec<_> = s.chars().collect();
        for (i, &ch) in chars.iter().enumerate().rev() {
            if ch == c {
                prev = i as i32;
            }
            ans[i] = ans[i].min(prev - (i as i32));
        }

        ans
    }
}
