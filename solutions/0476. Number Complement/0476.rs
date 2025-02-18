// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn find_complement(mut num: i32) -> i32 {
        let mut mask = !0;

        while mask & num != 0 {
            mask <<= 1;
        }

        !num & !mask
    }
}
