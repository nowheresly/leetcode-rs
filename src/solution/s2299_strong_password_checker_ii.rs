
pub struct Solution {}

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }
        let ch = password.chars().collect::<Vec<char>>();
        for i in 1..password.len() {
            if ch[i-1] == ch[i] {
                return false;
            }
        }
        if contains(&ch, "qwertzuiopasdfghjklyxcvbnm") == false {
            return false;
        }
        if contains(&ch, "QWERTZUIOPASDFGHJKLYXCVBNM") == false {
            return false;
        }
        if contains(&ch, "0123456789") == false {
            return false;
        }
        if contains(&ch, "!@#$%^&*()-+") == false {
            return false;
        }
        true
    }
}

fn contains(ch:&Vec<char>, data: &str) -> bool {
    for c in ch {
        if data.contains(*c) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(true, Solution::strong_password_checker_ii(String::from("IloveLe3tcode!")));


    }
}
