pub struct Solution {}

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut x = 0;
        let mut y = 0;

        for command in commands.iter() {
            match command.as_str() {
                "UP" => y -= 1,
                "DOWN" => y += 1,
                "LEFT" => x -= 1,
                "RIGHT" => x += 1,
                _ => ()
            };
        }
        (y * n) + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::final_position_of_snake(3, vec![String::from("DOWN"), String::from("RIGHT"), String::from("UP")]));
    }
}
