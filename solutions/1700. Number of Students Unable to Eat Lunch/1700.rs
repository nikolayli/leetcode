// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut count = vec![0, 0];

        for student in students {
            count[student as usize] += 1;
        }

        for i in 0..sandwiches.len() {
            if count[sandwiches[i] as usize] == 0 {
                return (sandwiches.len() - i) as i32;
            }
            count[sandwiches[i] as usize] -= 1;
        }

        0
    }
}
