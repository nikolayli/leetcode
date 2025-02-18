// Time complexity: O(n+k)
// Space complexity: O(n)
impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = arr.len();
        let mut xors = vec![0; n + 1];
        let mut ans = Vec::with_capacity(queries.len());

        for i in 0..n {
            xors[i + 1] = xors[i] ^ arr[i];
        }

        for query in queries {
            let left = query[0] as usize;
            let right = query[1] as usize;
            ans.push(xors[left] ^ xors[right + 1]);
        }

        ans
    }
}
