// Time complexity: O(n⋅logn+m⋅logm)
// Space complexity: O(logm+logn)
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut content_children = 0;
        let mut cookie_index = 0;
        while cookie_index < s.len() && content_children < g.len() {
            if s[cookie_index] >= g[content_children] {
                content_children += 1;
            }
            cookie_index += 1;
        }
        content_children as i32
    }
}
