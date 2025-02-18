// Time complexity: O(n)
// Space complexity: O(n)
impl Solutions {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut counts = vec![0; n + 1];

        for &num in arr.iter() {
            counts[n.min(num as usize)] += 1;
        }

        let mut ans = 1;
        for num in 2..=n {
            ans = (ans + counts[num]).min(num as i32);
        }

        ans
    }
}
