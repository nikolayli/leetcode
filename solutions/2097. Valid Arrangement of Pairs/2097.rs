// Time complexity: O(V+E)
// Space complexity: O(V+E)
use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut out_degree: HashMap<i32, i32> = HashMap::new();
        let mut in_degree: HashMap<i32, i32> = HashMap::new();

        for pair in &pairs {
            let start = pair[0];
            let end = pair[1];
            graph.entry(start).or_insert_with(Vec::new).push(end);
            *out_degree.entry(start).or_insert(0) += 1;
            *in_degree.entry(end).or_insert(0) += 1;
        }

        fn get_start_node(
            graph: &HashMap<i32, Vec<i32>>,
            out_degree: &HashMap<i32, i32>,
            in_degree: &HashMap<i32, i32>,
            pairs: &Vec<Vec<i32>>,
        ) -> i32 {
            for &u in graph.keys() {
                if out_degree.get(&u).unwrap_or(&0) - in_degree.get(&u).unwrap_or(&0) == 1 {
                    return u;
                }
            }
            pairs[0][0]
        }

        fn euler(u: i32, graph: &mut HashMap<i32, Vec<i32>>, ans: &mut Vec<Vec<i32>>) {
            while let Some(v) = graph.get_mut(&u).and_then(|stack| stack.pop()) {
                euler(v, graph, ans);
                ans.push(vec![u, v]);
            }
        }

        let start_node = get_start_node(&graph, &out_degree, &in_degree, &pairs);
        euler(start_node, &mut graph, &mut ans);
        ans.reverse();
        ans
    }
}
