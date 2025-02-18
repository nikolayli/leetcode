// Time complexity: O(mn)
// Space complexity: O(n)
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let mut ans = 0;
        let mut hist = vec![0; matrix[0].len()];

        for row in matrix {
            for i in 0..row.len() {
                hist[i] = if row[i] == '0' { 0 } else { hist[i] + 1 };
            }
            ans = ans.max(Self::largest_rectangle_area(&hist));
        }

        ans
    }

    fn largest_rectangle_area(heights: &[i32]) -> i32 {
        let mut ans = 0;
        let mut stack = Vec::new();

        for i in 0..=heights.len() {
            while !stack.is_empty()
                && (i == heights.len() || heights[stack[stack.len() - 1]] > heights[i])
            {
                let h = heights[stack.pop().unwrap()];
                let w = if stack.is_empty() {
                    i
                } else {
                    i - stack[stack.len() - 1] - 1
                };
                ans = ans.max(h * w as i32);
            }
            stack.push(i);
        }

        ans
    }
}
