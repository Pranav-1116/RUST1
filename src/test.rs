//  Regular function
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

//  Tuple struct
#[derive(Debug, PartialEq)]
pub struct Point(i32, i32);

// Implementation block
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn move_by(&mut self, ex: i32, ey: i32) {
        self.0 += ex;
        self.1 += ey;
    }
}

//  Optional runtime function
pub fn run() {
    let mut p = Point::new(1, 2);
    p.move_by(3, 4);
    println!("{:?}", p);
}

//  Tests (always at module level, usually at bottom)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5); // fixed expected value
    }

    #[test]
    fn test_creation() {
        let p = Point::new(4, 5);
        assert_eq!(p.x(), 4);
        assert_eq!(p.y(), 5);
    }

    #[test]
    fn test_move() {
        let mut p = Point::new(1, 2);
        p.move_by(3, 4);
        assert_eq!(p, Point(4, 6));
    }
}
