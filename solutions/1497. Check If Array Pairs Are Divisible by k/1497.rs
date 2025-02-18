// Time complexity: O(n)
// Space complexity: O(k)
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut count = vec![0; k as usize];

        for &a in &arr {
            let mod_val = a % k;
            let index = if mod_val < 0 { (mod_val + k) } else { mod_val };
            count[index as usize] += 1;
        }
        if count[0] % 2 != 0 {
            return false;
        }
        for i in 1..=(k / 2) {
            if count[i as usize] != count[(k - i) as usize] {
                return false;
            }
        }

        true
    }
}
