// Time complexity: O(mn)
// Space complexity: O(mn)
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut ans = vec![vec![-1; n as usize]; m as usize];

        let mut x = 0;
        let mut y = 0;
        let mut d = 0;

        let mut curr = head;
        while let Some(mut node) = curr {
            ans[x][y] = node.val;
            curr = node.next.take();

            let (dx, dy) = dirs[d];
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);

            if nx < 0
                || nx >= m as i32
                || ny < 0
                || ny >= n as i32
                || ans[nx as usize][ny as usize] != -1
            {
                d = (d + 1) % 4;
            }

            x = (x as i32 + dirs[d].0) as usize;
            y = (y as i32 + dirs[d].1) as usize;
        }

        ans
    }
}
