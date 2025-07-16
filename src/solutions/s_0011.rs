#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut l = 0;
        let mut r = height.len() - 1;

        while l < r {
            let min_height = height[l].min(height[r]);
            let width = (r - l) as i32;
            let area = min_height * width;
            ans = ans.max(area);

            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! max_area_test {
        ($name:ident: $height:expr => $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(Solution::max_area($height.into()), $expected);
            }
        };
    }

    max_area_test!(case1: [1, 8, 6, 2, 5, 4, 8, 3, 7] => 49);
    max_area_test!(case2: [1, 1] => 1);
}
