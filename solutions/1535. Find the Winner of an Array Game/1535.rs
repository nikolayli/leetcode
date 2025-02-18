// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let max_element = *arr.iter().max().unwrap();

        let mut curr = arr[0];
        let mut winstreak = 0;

        for i in 1..arr.len() {
            let opponent = arr[i];

            if curr > opponent {
                winstreak += 1;
            } else {
                curr = opponent;
                winstreak = 1;
            }

            if winstreak == k || curr == max_element {
                return curr;
            }
        }

        -1
    }
}
