// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        fn dfs(
            i: usize,
            head_id: usize,
            manager: &Vec<i32>,
            inform_time: &Vec<i32>,
            mem: &mut HashMap<usize, i32>,
        ) -> i32 {
            if let Some(&val) = mem.get(&i) {
                return val;
            }
            if i == head_id {
                return 0;
            }
            let parent = manager[i] as usize;
            let res = inform_time[parent] + dfs(parent, head_id, manager, inform_time, mem);
            mem.insert(i, res);

            res
        }

        let mut ans = 0;
        let mut mem = HashMap::new();
        for i in 0..n as usize {
            ans = ans.max(dfs(i, head_id as usize, &manager, &inform_time, &mut mem));
        }

        ans
    }
}
