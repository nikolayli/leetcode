// Time complexity: O(log₁₀(n))
// Space complexity: O(log₁₀(n))
impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let below_twenty = vec![
            "",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];

        let tens = vec![
            "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];

        fn helper(num: i32, below_twenty: &[&str], tens: &[&str]) -> String {
            let s = if num < 20 {
                below_twenty[num as usize].to_string()
            } else if num < 100 {
                format!(
                    "{} {}",
                    tens[(num / 10) as usize],
                    below_twenty[(num % 10) as usize]
                )
            } else if num < 1000 {
                format!(
                    "{} Hundred {}",
                    helper(num / 100, below_twenty, tens),
                    helper(num % 100, below_twenty, tens)
                )
            } else if num < 1_000_000 {
                format!(
                    "{} Thousand {}",
                    helper(num / 1000, below_twenty, tens),
                    helper(num % 1000, below_twenty, tens)
                )
            } else if num < 1_000_000_000 {
                format!(
                    "{} Million {}",
                    helper(num / 1_000_000, below_twenty, tens),
                    helper(num % 1_000_000, below_twenty, tens)
                )
            } else {
                format!(
                    "{} Billion {}",
                    helper(num / 1_000_000_000, below_twenty, tens),
                    helper(num % 1_000_000_000, below_twenty, tens)
                )
            };

            s.trim().to_string()
        }

        helper(num, &below_twenty, &tens)
    }
}
