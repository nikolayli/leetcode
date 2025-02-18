// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut words_freq: Vec<i32> = words
            .iter()
            .map(|word| Self::min_char_count(word))
            .collect();
        words_freq.sort_unstable();

        for query in queries {
            let count = Self::min_char_count(&query);
            let index = words_freq.iter().filter(|&&x| x > count).count();
            ans.push(index as i32);
        }

        ans
    }

    fn min_char_count(word: &String) -> i32 {
        let mut count = 0;
        let mut current_char = b'z' + 1;

        for &ch in word.as_bytes() {
            if ch < current_char {
                current_char = ch;
                count = 1;
            } else if ch == current_char {
                count += 1;
            }
        }

        count
    }
}
