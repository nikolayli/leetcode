// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n = boxes.len();
        let mut ans = vec![0; n];
        let boxes: Vec<char> = boxes.chars().collect();

        let mut count = 0;
        let mut moves = 0;
        for i in 0..n {
            ans[i] += moves;
            count += (boxes[i] as i32) - ('0' as i32);
            moves += count;
        }

        count = 0;
        moves = 0;
        for i in (0..n).rev() {
            ans[i] += moves;
            count += (boxes[i] as i32) - ('0' as i32);
            moves += count;
        }

        ans
    }
}
