// Time complexity: O(m+n)
// Space complexity: O(1)
impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut ans = 0i64;
        let mut dist = vec![vec![i32::MAX; 26]; 26];

        for ((&a, &b), &c) in original.iter().zip(changed.iter()).zip(cost.iter()) {
            let u = (a as u8 - b'a') as usize;
            let v = (b as u8 - b'a') as usize;
            dist[u][v] = dist[u][v].min(c);
        }

        for k in 0..26 {
            for i in 0..26 {
                if dist[i][k] < i32::MAX {
                    for j in 0..26 {
                        if dist[k][j] < i32::MAX {
                            dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                        }
                    }
                }
            }
        }

        for (s, t) in source.bytes().zip(target.bytes()) {
            if s == t {
                continue;
            }
            let u = (s - b'a') as usize;
            let v = (t - b'a') as usize;
            if dist[u][v] == i32::MAX {
                return -1;
            }
            ans += dist[u][v] as i64;
        }

        ans
    }
}
