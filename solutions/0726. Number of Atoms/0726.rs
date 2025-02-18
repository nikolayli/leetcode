// Time complexity: O(nlogn)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        fn parse(formula: &[u8], i: &mut usize) -> HashMap<String, i32> {
            let mut ans = HashMap::new();
            let n = formula.len();

            while *i < n {
                if formula[*i] == b'(' {
                    *i += 1;
                    let sub_count = parse(formula, i);
                    for (elem, freq) in sub_count {
                        *ans.entry(elem).or_insert(0) += freq;
                    }
                } else if formula[*i] == b')' {
                    *i += 1;
                    let mut num_start = *i;
                    while *i < n && formula[*i].is_ascii_digit() {
                        *i += 1;
                    }
                    let factor = if num_start < *i {
                        std::str::from_utf8(&formula[num_start..*i])
                            .unwrap()
                            .parse::<i32>()
                            .unwrap()
                    } else {
                        1
                    };
                    for freq in ans.values_mut() {
                        *freq *= factor;
                    }
                    return ans;
                } else if formula[*i].is_ascii_uppercase() {
                    let elem_start = *i;
                    *i += 1;
                    while *i < n && formula[*i].is_ascii_lowercase() {
                        *i += 1;
                    }
                    let elem = std::str::from_utf8(&formula[elem_start..*i])
                        .unwrap()
                        .to_string();
                    let mut num_start = *i;
                    while *i < n && formula[*i].is_ascii_digit() {
                        *i += 1;
                    }
                    let num = if num_start < *i {
                        std::str::from_utf8(&formula[num_start..*i])
                            .unwrap()
                            .parse::<i32>()
                            .unwrap()
                    } else {
                        1
                    };
                    *ans.entry(elem).or_insert(0) += num;
                }
            }

            ans
        }

        let mut i = 0;
        let formula_bytes = formula.as_bytes();
        let count = parse(formula_bytes, &mut i);

        let mut elements: Vec<_> = count.into_iter().collect();
        elements.sort_by(|a, b| a.0.cmp(&b.0));

        let mut ans = String::new();
        for (elem, freq) in elements {
            ans.push_str(&elem);
            if freq > 1 {
                ans.push_str(&freq.to_string());
            }
        }

        ans
    }
}
