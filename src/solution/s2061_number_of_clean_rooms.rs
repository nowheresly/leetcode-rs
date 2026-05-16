
pub struct Solution {}

impl Solution {
    pub fn number_of_clean_rooms(room: Vec<Vec<i32>>) -> i32 {
        let m = room.len() as i32;
        let n = room[0].len() as i32;

        let dx:Vec<i32> = vec![0, 1, 0, -1];
        let dy:Vec<i32> = vec![1, 0, -1, 0];

        let mut visited = vec![vec![vec![false;4];n as usize];m as usize];

        let mut cleaned = vec![vec![false;n as usize];m as usize];

        let mut x:i32 = 0;
        let mut y:i32 = 0;
        let mut dir = 0; // right = 0; down = 1; left = 2; up = 3
        let mut clean_count = 0;

        while visited[x as usize][y as usize][dir] == false {
            visited[x as usize][y as usize][dir] = true;

            if cleaned[x as usize][y as usize] == false {
                cleaned[x as usize][y as usize] = true;
                clean_count += 1;
            }

            let nx:i32 = x + dx[dir];
            let ny:i32 = y + dy[dir];

            if nx >= 0 && nx < m && ny >= 0 && ny < n && room[nx as usize][ny as usize] == 0 {
                x = nx;
                y = ny;
            } else {
                dir = (dir + 1) % 4;
            }
        }
        clean_count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(7, Solution::number_of_clean_rooms(vec![vec![0,0,0],vec![1,1,0],vec![0,0,0]]));

    }
}
