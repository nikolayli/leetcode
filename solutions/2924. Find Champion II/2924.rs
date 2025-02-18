// Time complexity: O(n+m)
// Space complexity: O(n)
impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut in_degrees = vec![0; n];

        for edge in edges {
            if let Some(&v) = edge.get(1) {
                in_degrees[v as usize] += 1;
            }
        }

        let zero_count = in_degrees.iter().filter(|&&x| x == 0).count();

        if zero_count > 1 {
            -1
        } else {
            in_degrees
                .iter()
                .position(|&x| x == 0)
                .map_or(-1, |pos| pos as i32)
        }
    }
}
