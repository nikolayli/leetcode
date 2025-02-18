// Time complexity: O(nlog(min(A,B)))
// Space complexity: O(n)
use regex::Regex;
use std::cmp::min;
use std::str::FromStr;

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let re = Regex::new(r"[+-]?[0-9]+").unwrap();
        let ints: Vec<i32> = re
            .find_iter(&expression)
            .map(|mat| i32::from_str(mat.as_str()).unwrap())
            .collect();

        let mut a = 0;
        let mut b = 1;

        for i in (0..ints.len()).step_by(2) {
            let num = ints[i];
            let denom = ints[i + 1];
            a = a * denom + num * b;
            b *= denom;
            let g = gcd(a, b);
            a /= g;
            b /= g;
        }

        format!("{}/{}", a, b)
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}
