// Time complexity: O(n + trust)
// Space complexity: O(n)
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![0; (n + 1) as usize];

        for t in trust {
            count[t[0] as usize] -= 1;
            count[t[1] as usize] += 1;
        }

        for i in 1..(n + 1) {
            if count[i as usize] == n - 1 {
                return i;
            }
        }

        -1
    }
}
