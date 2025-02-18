// Time complexity: O(n)
// Space complexity: O(n)
use std::ops::{BitAnd, BitOr};
use std::str::Chars;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        fn dfs(chars: &mut Chars, end: char) -> bool {
            let mut exps = Vec::new();
            let mut op = ' ';
            let mut layer = 0;

            while let Some(c) = chars.next() {
                match c {
                    't' | 'f' if layer == 0 => return c == 't',
                    '!' | '&' | '|' if layer == 0 => op = c,
                    '(' => {
                        layer += 1;
                        if layer == 1 {
                            exps.push(dfs(chars, ')'));
                        }
                    }
                    ')' => {
                        layer -= 1;
                        if layer == 0 {
                            break;
                        }
                    }
                    ',' if layer == 1 => exps.push(dfs(chars, ')')),
                    _ => {}
                }
            }

            match op {
                '|' => exps.into_iter().fold(false, BitOr::bitor),
                '&' => exps.into_iter().fold(true, BitAnd::bitand),
                '!' => !exps[0],
                _ => unreachable!(),
            }
        }

        dfs(&mut expression.chars(), '\0')
    }
}
