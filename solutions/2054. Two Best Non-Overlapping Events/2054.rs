// Time complexity: O(sort)
// Space complexity: O(n)
impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut max_value = 0;
        let mut evts = Vec::new();

        for event in events {
            let (s, e, v) = (event[0], event[1], event[2]);
            evts.push((s, true, v));
            evts.push((e + 1, false, v));
        }

        evts.sort_unstable();

        for &(_, is_start, value) in &evts {
            if is_start {
                ans = ans.max(value + max_value);
            } else {
                max_value = max_value.max(value);
            }
        }

        ans
    }
}
