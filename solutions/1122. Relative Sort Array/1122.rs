// Time complexity: O(n+m)
// Space complexity: O(n)
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; 1001];
        let mut ans = Vec::new();

        for &a in &arr1 {
            count[a as usize] += 1;
        }

        for &a in &arr2 {
            while count[a as usize] > 0 {
                ans.push(a);
                count[a as usize] -= 1;
            }
        }

        for num in 0..1001 {
            while count[num] > 0 {
                ans.push(num as i32);
                count[num] -= 1;
            }
        }

        ans
    }
}
