// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        colors.as_bytes().windows(3).fold(0, |acc, w| {
            acc + match w {
                b"AAA" => 1,
                b"BBB" => -1,
                _ => 0,
            }
        }) > 0
    }
}
