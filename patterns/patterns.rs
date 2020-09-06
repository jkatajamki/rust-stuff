fn main() {
  let favourite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favourite_color {
    println!("Using  your favourite color, {}, as teh background", color);
  } else if is_tuesday {
    println!("Tuesday is green day!");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Using purple as the background colour");
    } else {
      println!("Using orange as the background colour")
    }
  } else {
    println!("Using blue as the background colour");
  }

  let mut stack = Vec::new();

  stack.push(1);
  stack.push(2);
  stack.push(3);

  while let Some(top) = stack.pop() {
    println!("{}", top);
  }
}
