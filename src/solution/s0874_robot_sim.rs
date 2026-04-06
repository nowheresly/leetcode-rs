use std::collections::{HashSet};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Point {
    x:i32,
    y:i32
}


pub struct Solution {}

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut obs = HashSet::new();
        for i in 0..obstacles.len() {
            let x = obstacles[i][0];
            let y = obstacles[i][1];
            obs.insert(Point{x,y});
        }
        let mut cur = Point{x:0, y:0};
        let mut heading = 'N';
        let mut res = 0;
        for com in commands {
            if com < 0 {
                heading = turn(heading, com);
                continue;
            }
            for _ in 0..com {
                let prev = cur.clone();
                cur = move_robot(&cur, heading);
                if obs.contains(&cur) {
                    cur = prev;
                    break;
                }
                res = res.max(cur.x * cur.x + cur.y * cur.y);
            }
        }
        res
    }
}

fn move_robot(cur: &Point, heading: char) -> Point {
    if heading == 'N' {
        Point{x:cur.x, y:cur.y + 1}
    }
    else if heading == 'S' {
        Point{x:cur.x, y:cur.y - 1}
    }
    else if heading == 'L' {
        Point{x:cur.x - 1, y:cur.y}
    } else {
        Point{x:cur.x + 1, y:cur.y}
    }
}

fn turn(heading: char, command: i32) -> char  {
    if heading == 'N' {
        if command == -1 {
            'R'
        } else {
            'L'
        }
    }
    else if heading == 'S' {
        if command == -1 {
            'L'
        } else {
            'R'
        }
    }
    else if heading == 'L' {
        if command == -1 {
            'N'
        } else {
            'S'
        }
    }
    else {
        if command == -1 {
            'S'
        } else {
            'N'
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(65, Solution::robot_sim(vec![4,-1,4,-2,4], vec![vec![2,4]]));

    }
}
