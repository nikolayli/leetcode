// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut defanged_address = String::new();

        for c in address.chars() {
            if c == '.' {
                defanged_address.push_str("[.]");
            } else {
                defanged_address.push(c);
            }
        }

        defanged_address
    }
}
