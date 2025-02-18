// Time complexity: O((|queries| + |blocks|) * log(50001))
// Space complexity: O(50001 + |queries| + |blocks|)
use std::cmp;
use std::collections::BTreeSet;

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut segment_tree = vec![0; cmp::min(50000, 3 * queries.len()) + 1];
        let mut ans: Vec<bool> = Vec::new();
        let mut blocks: BTreeSet<i32> = BTreeSet::new();

        for query in &queries {
            if query[0] == 1 {
                blocks.insert(query[1]);
            }
        }

        let mut blocks: Vec<i32> = blocks.into_iter().collect();
        blocks.insert(0, 0);

        Self::update_blocks(&mut blocks, &mut segment_tree);

        for query in queries.iter().rev() {
            let pos = match blocks.binary_search(&query[1]) {
                Ok(p) => p,
                Err(p) => p,
            };
            if query[0] == 1 {
                if pos + 1 < blocks.len() {
                    Self::update_segment_tree(
                        &mut segment_tree,
                        blocks[pos + 1] as usize,
                        blocks[pos + 1] - blocks[pos - 1],
                    );
                }
                blocks.remove(pos);
            } else {
                let can_place = query[1] - blocks[pos - 1] >= query[2]
                    || Self::query_segment_tree(&segment_tree, query[1] as isize) >= query[2];
                ans.push(can_place);
            }
        }

        ans.reverse();
        ans
    }

    fn update_segment_tree(segment_tree: &mut Vec<i32>, mut pos: usize, value: i32) {
        while pos < segment_tree.len() {
            segment_tree[pos] = cmp::max(segment_tree[pos], value);
            pos |= pos + 1;
        }
    }

    fn query_segment_tree(segment_tree: &Vec<i32>, mut pos: isize) -> i32 {
        let mut max_value = 0;
        while pos >= 0 {
            max_value = cmp::max(max_value, segment_tree[pos as usize]);
            pos = (pos & (pos + 1)) - 1;
        }

        max_value
    }

    fn update_blocks(blocks: &mut Vec<i32>, segment_tree: &mut Vec<i32>) {
        for i in 1..blocks.len() {
            Self::update_segment_tree(segment_tree, blocks[i] as usize, blocks[i] - blocks[i - 1]);
        }
    }
}
