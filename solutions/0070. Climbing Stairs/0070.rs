// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let mut first = 1;
        let mut second = 2;
        for i in 3..=n {
            let third = first + second;
            first = second;
            second = third;
        }

        return second;
    }
}
