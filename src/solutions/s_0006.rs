#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows as usize;
        let mut rows = vec![String::new(); num_rows];
        let mut k = 0;
        let mut dir = -1;

        for c in s.chars() {
            rows[k].push(c);
            if k == 0 || k == num_rows - 1 {
                dir *= -1;
            }
            k = (k as isize + dir) as usize;
        }

        rows.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! convert_test {
        ($name:ident: $s:expr, $num_rows:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(
                    Solution::convert($s.to_string(), $num_rows),
                    $expected.to_string()
                );
            }
        };
    }

    convert_test!(case1: "PAYPALISHIRING", 3 => "PAHNAPLSIIGYIR");
    convert_test!(case2: "PAYPALISHIRING", 4 => "PINALSIGYAHRPI");
    convert_test!(case3: "A", 1 => "A");
}
