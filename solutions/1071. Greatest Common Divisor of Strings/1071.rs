// Time complexity: O(m+n)
// Space complexity: O(m+n)
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.clone() + &str2 != str2.clone() + &str1 {
            return String::new();
        }

        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        let gcd_length = gcd(str1.len(), str2.len());
        str1[..gcd_length].to_string()
    }
}
