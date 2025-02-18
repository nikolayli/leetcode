// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let mut time = time % ((n - 1) * 2);
        if time < n {
            return 1 + time;
        }

        n - (time - (n - 1))
    }
}
