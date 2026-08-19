pub struct Solution {}

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut reserved_seats = reserved_seats;
        reserved_seats.sort_unstable();
        let mut res = 2 * n;
        let mut i = 0;
        while i < reserved_seats.len() {
            let row = reserved_seats[i][0];
            let mut left_free = true;
            let mut middle_free = true;
            let mut right_free = true;


            while i < reserved_seats.len() && reserved_seats[i][0] == row {
                let seat = reserved_seats[i][1];

                if seat >= 2 && seat <= 5 {
                    left_free = false;
                }
                if seat >= 4 && seat <= 7 {
                    middle_free = false;
                }
                if seat >= 6 && seat <= 9 {
                    right_free = false;
                }
                i += 1;
            }
            if left_free && right_free {
            } else if left_free || right_free || middle_free {
                res -= 1;
            } else {
                res -= 2;
            }
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
            2,
            Solution::max_number_of_families(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]])
        );
    }
}
