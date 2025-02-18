// Time complexity: O(⁡n)
// Space complexity: O(1)
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut count5 = 0;
        let mut count10 = 0;

        for &bill in &bills {
            match bill {
                5 => count5 += 1,
                10 => {
                    if count5 > 0 {
                        count5 -= 1;
                        count10 += 1;
                    } else {
                        return false;
                    }
                }
                20 => {
                    if count10 > 0 && count5 > 0 {
                        count10 -= 1;
                        count5 -= 1;
                    } else if count5 >= 3 {
                        count5 -= 3;
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        true
    }
}
