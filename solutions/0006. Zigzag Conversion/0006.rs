// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows as usize;
        let mut rows = vec![String::new(); num_rows];
        let mut k = 0;
        let mut direction = -1;

        for c in s.chars() {
            rows[k].push(c);
            if k == 0 || k == num_rows - 1 {
                direction *= -1;
            }
            k = (k as isize + direction) as usize;
        }

        rows.concat()
    }
}
