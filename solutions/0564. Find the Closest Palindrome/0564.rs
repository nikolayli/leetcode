// Time complexity: O(logn)
// Space complexity: O(logn)
impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let (prev_palindrome, next_palindrome) = Self::get_palindromes(&n);
        let num = n.parse::<i64>().unwrap();
        if (prev_palindrome - num).abs() <= (next_palindrome - num).abs() {
            prev_palindrome.to_string()
        } else {
            next_palindrome.to_string()
        }
    }

    fn get_palindromes(s: &str) -> (i64, i64) {
        let num = s.parse::<i64>().unwrap();
        let sz = s.len();
        let mut palindromes = Vec::new();
        let half = &s[0..(sz + 1) / 2];
        let reversed_half: String = half
            .chars()
            .take(sz / 2)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        let candidate = format!("{}{}", half, reversed_half).parse::<i64>().unwrap();

        if candidate < num {
            palindromes.push(candidate);
        } else {
            let prev_half = (half.parse::<i64>().unwrap() - 1).to_string();
            let reversed_prev_half: String = prev_half
                .chars()
                .take(sz / 2)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            if sz % 2 == 0 && prev_half.parse::<i64>().unwrap() == 0 {
                palindromes.push(9);
            } else if sz % 2 == 0 && prev_half == "9" {
                palindromes.push(
                    format!("{}9{}", prev_half, reversed_prev_half)
                        .parse::<i64>()
                        .unwrap(),
                );
            } else {
                palindromes.push(
                    format!("{}{}", prev_half, reversed_prev_half)
                        .parse::<i64>()
                        .unwrap(),
                );
            }
        }

        if candidate > num {
            palindromes.push(candidate);
        } else {
            let next_half = (half.parse::<i64>().unwrap() + 1).to_string();
            let reversed_next_half: String = next_half
                .chars()
                .take(sz / 2)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            palindromes.push(
                format!("{}{}", next_half, reversed_next_half)
                    .parse::<i64>()
                    .unwrap(),
            );
        }

        (palindromes[0], palindromes[1])
    }
}
