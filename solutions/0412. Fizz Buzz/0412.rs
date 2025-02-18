// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|n| match (n % 3, n % 5) {
                (0, 0) => "FizzBuzz".to_owned(),
                (0, _) => "Fizz".to_owned(),
                (_, 0) => "Buzz".to_owned(),
                _ => n.to_string(),
            })
            .collect()
    }
}
