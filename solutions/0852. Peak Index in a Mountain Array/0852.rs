// Time complexity: O(log⁡n)
// Space complexity: O(1)
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = arr.len() - 1;

        while l < r {
            let m = l + (r - l) / 2;
            if arr[m] >= arr[m + 1] {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l as i32
    }
}
