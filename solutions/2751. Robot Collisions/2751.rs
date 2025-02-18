// Time complexity: O(sort)
// Space complexity: O(n)
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Robot {
    index: usize,
    position: i32,
    health: i32,
    direction: char,
}

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut robots: Vec<Robot> = positions
            .into_iter()
            .zip(healths.into_iter())
            .zip(directions.chars())
            .enumerate()
            .map(|(index, ((position, health), direction))| Robot {
                index,
                position,
                health,
                direction,
            })
            .collect();
        robots.sort_unstable_by(|a, b| a.position.cmp(&b.position));
        let mut stack: Vec<Robot> = Vec::new();

        for mut robot in robots {
            if robot.direction == 'R' {
                stack.push(robot);
                continue;
            }
            while let Some(last) = stack.last_mut() {
                if last.direction == 'R' && robot.health > 0 {
                    if last.health == robot.health {
                        stack.pop();
                        robot.health = 0;
                    } else if last.health < robot.health {
                        stack.pop();
                        robot.health -= 1;
                    } else {
                        last.health -= 1;
                        robot.health = 0;
                    }
                } else {
                    break;
                }
            }
            if robot.health > 0 {
                stack.push(robot);
            }
        }

        stack.sort_unstable_by(|a, b| a.index.cmp(&b.index));
        stack.into_iter().map(|robot| robot.health).collect()
    }
}
