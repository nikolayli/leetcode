// Time complexity: O(1)
// Space complexity: O(1)
impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        if a == e {
            return if c == a && ((b < d && d < f) || (b > d && d > f)) {
                2
            } else {
                1
            };
        }
        if b == f {
            return if d == f && ((a < c && c < e) || (a > c && c > e)) {
                2
            } else {
                1
            };
        }
        if c + d == e + f {
            return if a + b == c + d && ((c < a && a < e) || (c > a && a > e)) {
                2
            } else {
                1
            };
        }
        if c - d == e - f {
            return if a - b == c - d && ((c < a && a < e) || (c > a && a > e)) {
                2
            } else {
                1
            };
        }
        2
    }
}
