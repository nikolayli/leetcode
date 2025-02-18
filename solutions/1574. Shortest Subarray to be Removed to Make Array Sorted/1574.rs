// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut l = 0;
        let mut r = n - 1;

        while l < n - 1 && arr[l + 1] >= arr[l] {
            l += 1;
        }
        while r > 0 && arr[r - 1] <= arr[r] {
            r -= 1;
        }
        let mut ans = (n - 1 - l).min(r);

        let mut i = l as isize;
        let mut j = (n - 1) as isize;
        while i >= 0 && j >= r as isize && j > i {
            if arr[i as usize] <= arr[j as usize] {
                j -= 1;
            } else {
                i -= 1;
            }
            ans = ans.min((j - i) as usize);
        }

        ans as i32
    }
}
