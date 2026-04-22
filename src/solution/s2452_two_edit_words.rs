pub struct Solution {}

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        let mut res = vec![];
        for q in queries.iter() {
            for d in dictionary.iter() {
                if is_match(q, d) {
                    res.push(q.clone());
                    break;
                }
            }
        }
        res
    }
}

fn is_match(q:&str, d:&str) -> bool {
    let mut diff_count = 0;
    for i in 0..q.len() {
        if q.as_bytes()[i] != d.as_bytes()[i] {
            diff_count += 1;
            if diff_count > 2 {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![
                String::from("word"),
                String::from("note"),
                String::from("wood")
            ],
            Solution::two_edit_words(
                vec![
                    String::from("word"),
                    String::from("note"),
                    String::from("ants"),
                    String::from("wood")
                ],
                vec![
                    String::from("wood"),
                    String::from("joke"),
                    String::from("moat")
                ]
            )
        );
    }
}
