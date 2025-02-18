// Time complexity: O(nm)
// Space complexity: O(1)
impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();

        for row in &matrix {
            let min_index = row
                .iter()
                .position(|&x| x == *row.iter().min().unwrap())
                .unwrap();
            if row[min_index] == matrix.iter().map(|row| row[min_index]).max().unwrap() {
                ans.push(row[min_index]);
            }
        }

        ans
    }
}
