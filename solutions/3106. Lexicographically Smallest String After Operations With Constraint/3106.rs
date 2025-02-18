// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn get_smallest_string(s: String, mut k: i32) -> String {
        let mut res = String::new();

        for c in s.chars() {
            let l = c as i32 - 'a' as i32;
            let r = 'z' as i32 - c as i32 + 1;
            let min_distance = l.min(r);

            if k >= min_distance {
                k -= min_distance;
                res.push(if min_distance <= l { 'a' } else { 'z' });
            } else {
                let new_char = ((c as i32 - k) as u8) as char;
                res.push(new_char);
                k = 0;
            }
        }

        res
    }
}
