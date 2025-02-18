// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable();
        let mut work_until = 0;
        let mut ans = 0;

        for meeting in meetings {
            if meeting[0] > work_until {
                ans += meeting[0] - work_until - 1;
            }
            work_until = work_until.max(meeting[1]);
        }

        ans + days - work_until
    }
}
