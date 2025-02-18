// Time complexity: O(sort)
// Space complexity: O(sort)
impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();

        let mut ans = 0;
        let mut i = 0;
        let mut j = people.len() - 1;

        while i < j {
            if people[i] + people[j] <= limit {
                i += 1;
                j -= 1;
            } else {
                j -= 1;
            }
            ans += 1;
        }
        if i == j {
            ans += 1;
        }

        ans
    }
}
