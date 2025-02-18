// Time complexity: O(nk)
// Space complexity: O(1)
impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        let mut common_count = vec![i32::MAX; 26];

        for s in &a {
            let mut count = vec![0; 26];
            for c in s.chars() {
                count[(c as usize) - ('a' as usize)] += 1;
            }
            for i in 0..26 {
                common_count[i] = common_count[i].min(count[i]);
            }
        }

        for i in 0..26 {
            for _ in 0..common_count[i] {
                ans.push(((('a' as u8) + i as u8) as char).to_string());
            }
        }

        ans
    }
}
