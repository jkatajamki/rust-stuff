use std::fmt;

fn repeat_asterisk(len: usize, times: usize) -> String {
    "*".repeat(len + times)
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();

        println!("{}", repeat_asterisk(len, 4));
        println!("*{}*", repeat_asterisk(len, 2));
        println!("* {} *", output);
        println!("*{}*", repeat_asterisk(len, 2));
        println!("{}", repeat_asterisk(len, 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {

}

fn main() {

}
