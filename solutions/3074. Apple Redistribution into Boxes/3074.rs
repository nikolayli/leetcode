// Time complexity: O(n+mlogm)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut sum: i32 = apple.iter().sum();
        let mut i: usize = 0;
        capacity.sort_unstable_by(|a, b| b.cmp(a));

        for &c in capacity.iter() {
            if sum <= 0 {
                break;
            }
            sum -= c;
            i += 1;
        }

        i as i32
    }
}
