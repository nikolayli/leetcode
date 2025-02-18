// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn longest_diverse_string(mut a: i32, mut b: i32, mut c: i32) -> String {
        let mut ans = String::new();
        let mut curra = 0;
        let mut currb = 0;
        let mut currc = 0;
        let mut total = a + b + c;

        for _ in 0..total {
            if (a >= b && a >= c && curra != 2) || (a > 0 && (currb == 2 || currc == 2)) {
                ans.push('a');
                a -= 1;
                curra += 1;
                currb = 0;
                currc = 0;
            } else if (b >= a && b >= c && currb != 2) || (b > 0 && (curra == 2 || currc == 2)) {
                ans.push('b');
                b -= 1;
                currb += 1;
                curra = 0;
                currc = 0;
            } else if (c >= a && c >= b && currc != 2) || (c > 0 && (curra == 2 || currb == 2)) {
                ans.push('c');
                c -= 1;
                currc += 1;
                curra = 0;
                currb = 0;
            }
        }

        ans
    }
}
