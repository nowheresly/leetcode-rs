
struct Robot {
    width:i32,
    height: i32,
    perimeter:i32,
    pos:i32,
    has_moved:bool
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {

    fn new(width: i32, height: i32) -> Self {
        Self{
            width,
            height,
            perimeter: 2 * (width + height - 2),
            pos: 0,
            has_moved: false
        }
    }

    fn step(&mut self, num: i32) {
        self.has_moved = true;
        self.pos = (self.pos + num) % self.perimeter;
    }

    fn get_pos(&self) -> Vec<i32> {
        let h = self.height;
        let w = self.width;
        // bottom edge
        if self.pos <= w - 1 {
            return vec![self.pos, 0];
        }
        // right edge
        if self.pos <= (w-1) + (h-1) {
            return vec![w - 1, self.pos - w + 1];
        }
        // top edge
        if self.pos <= 2*(w-1) + (h-1) {
            return vec![w - 1 - (self.pos - ((w - 1) + (h - 1))), h - 1];
        }
        // left edge
        vec![0, h - 1 - (self.pos - (2 * (w - 1) + (h - 1)))]
    }

    fn get_dir(&self) -> String {
        if self.pos == 0 {
            return if self.has_moved { String::from("South") } else { String::from("East") };
        }
        // Bottom edge
        else if self.pos <= self.width - 1 {
            return String::from("East");
        }
        // Right edge
        else if self.pos <= (self.width - 1) + (self.height - 1) {
            return String::from("North");
        }
        // Top edge
        else if self.pos <= 2 * (self.width - 1) + (self.height - 1) {
            return String::from("West");
        }
        // Left edge
        else {
            return String::from("South");
        }
    }
}

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let width = 6;
        let height = 3;

        let mut obj = Robot::new(width, height);
        obj.step(2);
        let ret_2: Vec<i32> = obj.get_pos();
        assert_eq!(vec![2, 0], ret_2);
        let ret_3: String = obj.get_dir();
        assert_eq!(String::from("East"), ret_3);
    }
}
