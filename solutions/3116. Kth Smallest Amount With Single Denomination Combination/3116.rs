// Time complexity: O(2^n*logk)
// Space complexity: O(2^n)
impl Solution {
    pub fn find_kth_smallest(mut coins: Vec<i32>, k: i32) -> i64 {
        let k = k as i64;
        let lcms = Self::get_size_to_lcms(&coins);
        let mut l = 0;
        let mut r = 1_000_000_000_000_000;

        while l < r {
            let m = l + (r - l) / 2;
            if Self::num_denominations_no_greater_than(&lcms, m) >= k {
                r = m;
            } else {
                l = m + 1;
            }
        }

        l
    }

    fn num_denominations_no_greater_than(size_to_lcms: &Vec<Vec<i64>>, m: i64) -> i64 {
        let mut res = 0;
        for i in 1..size_to_lcms.len() {
            if i % 2 == 1 {
                for lcm in &size_to_lcms[i] {
                    res += m / lcm;
                }
            } else {
                for lcm in &size_to_lcms[i] {
                    res -= m / lcm;
                }
            }
        }

        res
    }

    fn get_size_to_lcms(coins: &Vec<i32>) -> Vec<Vec<i64>> {
        let mut res = vec![Vec::new(); coins.len() + 1];

        for i in 1..(1 << coins.len()) {
            let mut pos = 0;
            let mut size_to_lcms = Vec::new();
            let mut i_copy = i;
            while i_copy > 0 {
                if i_copy % 2 == 1 {
                    size_to_lcms.push(coins[pos]);
                }
                pos += 1;
                i_copy /= 2;
            }

            let mut lcm = size_to_lcms[0] as i64;
            for i in 1..size_to_lcms.len() {
                lcm = Self::lcm(lcm, size_to_lcms[i] as i64);
            }

            res[size_to_lcms.len()].push(lcm);
        }

        res
    }

    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            return a;
        }
        Self::gcd(b, a % b)
    }

    fn lcm(a: i64, b: i64) -> i64 {
        a / Self::gcd(a, b) * b
    }
}
