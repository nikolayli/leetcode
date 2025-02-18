// Time complexity: O(n+q)
// Space complexity: O(n+q)
impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let indices: Vec<i32> = nums
            .iter()
            .enumerate()
            .filter(|&(_i, &num)| num == x)
            .map(|(i, _num)| i as i32)
            .collect();
        queries
            .iter()
            .map(|&query| {
                if query as usize <= indices.len() {
                    indices[(query - 1) as usize]
                } else {
                    -1
                }
            })
            .collect()
    }
}
