// Time complexity: O(logn)
// Space complexity: O(1)
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut good: i32 = 0;
        let mut bad: i32 = n;

        while bad > good + 1 {
            let mid = good + ((bad - good) >> 1);
            if self.isBadVersion(mid) {
                bad = mid;
            } else {
                good = mid;
            }
        }
        return bad;
    }
}
