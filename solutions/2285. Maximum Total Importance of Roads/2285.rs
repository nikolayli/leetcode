// Time complexity: O(sort+m)
// Space complexity: O(n)
impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        let mut count = vec![0; n as usize];

        for road in roads {
            count[road[0] as usize] += 1;
            count[road[1] as usize] += 1;
        }

        count.sort_unstable();

        count
            .iter()
            .enumerate()
            .map(|(i, &c)| (i as i64 + 1) * c as i64)
            .sum()
    }
}
