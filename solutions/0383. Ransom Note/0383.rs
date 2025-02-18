// Time complexity: O(n+m)
// Space complexity: O(1)
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count = [0; 26];

        for c in magazine.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }

        for c in ransom_note.chars() {
            let index = (c as u8 - b'a') as usize;
            if count[index] == 0 {
                return false;
            }
            count[index] -= 1;
        }

        true
    }
}
