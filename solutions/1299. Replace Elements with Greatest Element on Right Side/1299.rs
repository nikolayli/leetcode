// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max_of_right = -1;

        for value in arr.iter_mut().rev() {
            let new_value = max_of_right;
            if *value > max_of_right {
                max_of_right = *value;
            }
            *value = new_value;
        }

        arr
    }
}
