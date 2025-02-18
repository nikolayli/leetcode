// Time complexity: O(nlog(max(m)))
// Space complexity: O(1)
impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut l = 1;
        let mut r = *quantities.iter().max().unwrap();

        while l < r {
            let m = (l + r) / 2;
            if Self::num_stores(m, &quantities) <= n {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l
    }

    fn num_stores(m: i32, quantities: &[i32]) -> i32 {
        quantities.iter().map(|&q| (q - 1) / m + 1).sum()
    }
}
