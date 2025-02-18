// Time complexity: O(mn)
// Space complexity: O(m)
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let mut i = 0;

        for num in target {
            while i < num - 1 {
                ans.push("Push".to_string());
                ans.push("Pop".to_string());
                i += 1;
            }

            ans.push("Push".to_string());
            i += 1;
        }

        ans
    }
}
