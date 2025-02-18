// Time complexity: O(n)
// Space complexity: O(n)
use std::collections::HashMap;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i64> = Vec::new();
        let mut op: HashMap<String, Box<dyn Fn(i64, i64) -> i64>> = HashMap::new();
        op.insert("+".to_string(), Box::new(|a, b| a + b));
        op.insert("-".to_string(), Box::new(|a, b| a - b));
        op.insert("*".to_string(), Box::new(|a, b| a * b));
        op.insert("/".to_string(), Box::new(|a, b| a / b));

        for token in tokens {
            if op.contains_key(&token) {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(op[&token](a, b));
            } else {
                stack.push(token.parse().unwrap());
            }
        }

        stack.pop().unwrap() as i32
    }
}
