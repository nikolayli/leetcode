// Time complexity: O(sort+mlogn)
// Space complexity: O(n)
impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable();
        items.dedup_by(|r, l| r[1] <= l[1]);
        queries
            .into_iter()
            .map(|q| match items.partition_point(|it| it[0] <= q) {
                0 => 0,
                i => items[i - 1][1],
            })
            .collect()
    }
}
