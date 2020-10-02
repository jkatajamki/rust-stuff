use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let point_a = Point {
        x: 1,
        y: 0,
    };
    let point_b = Point {
        x: 2,
        y: 3,
    };
    let point_c = Point {
        x: 3,
        y: 3,
    };

    assert_eq!(point_a + point_b, point_c);
}
