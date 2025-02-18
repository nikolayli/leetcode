// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n <= 2 {
            return if n == 0 { 0 } else { 1 };
        }

        let (mut a, mut b, mut c) = (0, 1, 1);
        for i in 3..=n {
            let t = a + b + c;
            a = b;
            b = c;
            c = t;
        }

        c
    }
}
