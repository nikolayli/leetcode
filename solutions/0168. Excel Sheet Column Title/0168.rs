// Time complexity: O(logn)
// Space complexity: O(logn)
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_number = column_number;
        let mut ans = String::new();

        while column_number > 0 {
            column_number -= 1;
            let remainder = (column_number % 26) as u8;
            let char_code = (remainder + b'A') as char;
            ans.insert(0, char_code);
            column_number /= 26;
        }

        ans
    }
}
