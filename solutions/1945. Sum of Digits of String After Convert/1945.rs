// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut ans = Self::convert(&s);
        for _ in 1..k {
            ans = Self::get_digit_sum(ans);
        }

        ans
    }

    fn convert(s: &str) -> i32 {
        s.chars()
            .map(|c| {
                let val = c as i32 - 'a' as i32 + 1;
                if val < 10 {
                    val
                } else {
                    val % 10 + val / 10
                }
            })
            .sum()
    }

    fn get_digit_sum(mut num: i32) -> i32 {
        let mut digit_sum = 0;
        while num > 0 {
            digit_sum += num % 10;
            num /= 10;
        }

        digit_sum
    }
}
