// Time complexity: O(n^26)
// Space complexity: O(n)
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut count = vec![0; 26];
        for t in tiles.chars() {
            count[(t as u8 - b'A') as usize] += 1;
        }

        fn dfs(count: &mut Vec<i32>) -> i32 {
            let mut possible_sequences = 0;

            for i in 0..26 {
                if count[i] == 0 {
                    continue;
                }
                count[i] -= 1;
                possible_sequences += 1 + dfs(count);
                count[i] += 1;
            }

            possible_sequences
        }

        dfs(&mut count)
    }
}
