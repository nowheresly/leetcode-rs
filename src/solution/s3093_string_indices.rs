pub struct Solution {}

#[derive(Clone)]
pub struct Trie {
    children: [Option<Box<Trie>>; 26],
    min:i32,
    index: i32
}

impl Trie {

    fn new() -> Self {
        const EMPTY_CHILD: Option<Box<Trie>> = None;
        Self {
            children: [EMPTY_CHILD; 26],
            min: i32::MAX,
            index: -1
        }
    }
    fn insert(&mut self, w:&str, idx:i32) {
        let mut cur = self;
        if cur.min > w.len() as i32 {
            cur.min = w.len() as i32;
            cur.index = idx;
        }
        for &b in w.as_bytes().iter().rev() {
            let char_idx = (b - b'a') as usize;
            cur = cur.children[char_idx].get_or_insert_with(|| {
                Box::new(Trie::new())
            });
            if cur.min > w.len() as i32 {
                cur.min = w.len() as i32;
                cur.index = idx;
            }
        }
    }

    fn find(&self, prefix:&str) -> i32 {
        let mut cur = self;
        for &b in prefix.as_bytes().iter().rev() {
            let char_idx = (b - b'a') as usize;
            if let Some(ref next) = cur.children[char_idx] {
                cur = next.as_ref();
            } else {
                break;
            }
        }
        cur.index
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let n = words_query.len();
        let mut res = vec![0; n];
        let mut trie = Trie::new();
        let mut idx = 0;
        for word in words_container {
            trie.insert(&word, idx);
            idx += 1;
        }
        for i in 0..n {
            let index = trie.find(&words_query[i]);
            res[i] = index;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            vec![1, 1, 1],
            Solution::string_indices(
                vec![
                    String::from("abcd"),
                    String::from("bcd"),
                    String::from("xbcd")
                ],
                vec![String::from("cd"), String::from("bcd"), String::from("xyz")]
            )
        );
    }
}
