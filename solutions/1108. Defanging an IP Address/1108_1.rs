// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}
