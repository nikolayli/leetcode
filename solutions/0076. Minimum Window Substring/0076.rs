// Time complexity: O(m+n)
// Space complexity: O(1)
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_bytes = s.as_bytes();

        let mut count: HashMap<u8, i32> = t.bytes().fold(HashMap::new(), |mut f, ch| {
            *f.entry(ch).or_default() += 1;
            f
        });

        let mut required = count.len() as i32;
        let (mut l, mut r) = (0, 0);
        let (mut best_left, mut min_length) = (0, s.len() + 1);

        while r < s_bytes.len() {
            if let Some(f) = count.get_mut(&s_bytes[r]) {
                *f -= 1;
                if *f == 0 {
                    required -= 1;
                }
            }
            r += 1;

            while l < s_bytes.len() && required <= 0 {
                if r - l < min_length {
                    best_left = l;
                    min_length = r - l;
                }
                if let Some(f) = count.get_mut(&s_bytes[l]) {
                    if *f == 0 {
                        required += 1;
                    }
                    *f += 1;
                }
                l += 1;
            }
        }

        if min_length <= s_bytes.len() {
            s[best_left..best_left + min_length].to_string()
        } else {
            String::new()
        }
    }
}
