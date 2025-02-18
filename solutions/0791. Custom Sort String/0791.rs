// Time complexity: O(∣order∣+∣s∣)
// Space complexity: O(26)
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut ans = String::new();
        let mut count = vec![0; 26];

        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        for c in order.chars() {
            while count[(c as u8 - b'a') as usize] > 0 {
                ans.push(c);
                count[(c as u8 - b'a') as usize] -= 1;
            }
        }

        for c in 'a'..='z' {
            while count[(c as u8 - b'a') as usize] > 0 {
                ans.push(c);
                count[(c as u8 - b'a') as usize] -= 1;
            }
        }

        ans
    }
}
