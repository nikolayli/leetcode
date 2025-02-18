// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;

        for (i, ticket) in tickets.iter().enumerate() {
            let curr = *ticket.min(&tickets[k as usize]);
            if i <= k as usize || &tickets[k as usize] > ticket {
                ans += curr;
            } else {
                ans += curr - 1;
            }
        }

        ans
    }
}
