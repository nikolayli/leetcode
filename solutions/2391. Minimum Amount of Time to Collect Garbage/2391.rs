// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let prefix: Vec<i32> = travel
            .iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect();

        let get_time = |c: char| -> i32 {
            let mut character_count = 0;
            let mut last_index = None;

            for (i, s) in garbage.iter().enumerate() {
                if s.contains(c) {
                    last_index = Some(i);
                }
                character_count += s.chars().filter(|&ch| ch == c).count() as i32;
            }

            character_count + last_index.map_or(0, |i| if i == 0 { 0 } else { prefix[i - 1] })
        };

        get_time('M') + get_time('P') + get_time('G')
    }
}
