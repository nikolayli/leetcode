// Time complexity: O(n)
// Space complexity: O(n)
impl Solution {
    pub fn clear_stars(s: String) -> String {
        let mut pos: Vec<Vec<usize>> = vec![Vec::new(); 26];
        let mut chars: Vec<char> = s.chars().collect();
        let mut to_replace = Vec::new();

        for (i, c) in chars.iter().enumerate() {
            if *c == '*' {
                for p in pos.iter_mut() {
                    if let Some(last) = p.pop() {
                        to_replace.push(last);
                        break;
                    }
                }
            } else {
                let index = (*c as usize) - ('a' as usize);
                pos[index].push(i);
            }
        }

        for i in to_replace {
            chars[i] = '*';
        }

        chars.into_iter().filter(|&c| c != '*').collect()
    }
}
