use std::io;

const CELSIUS_SCALE: &str = "C";
const FAHRENHEIT_SCALE: &str = "F";

fn convert_to_celsius(input: f64) -> f64 {
  (input - 32.) * 5. / 9.
}

fn convert_to_fahrenheit(input: f64) -> f64 {
  input * 1.8 + 32.
}

fn convert(input: f64, scale: &str) -> f64 {
  match scale {
    CELSIUS_SCALE => convert_to_celsius(input),
    FAHRENHEIT_SCALE => convert_to_fahrenheit(input),
    _ => 0.0,
  }
}

fn main() {
  println!("Enter the temperature you want to convert:");

  let mut temp_input = String::new();
  let mut scale_input = String::new();

  io::stdin().read_line(&mut temp_input)
    .expect("Failed to read line");

  let temp_degrees: f64 = temp_input
    .trim()
    .parse()
    .expect("Failed to convert input to float");

  println!("Enter the scale to which you want to convert (F for fahrenheit, C for celsius)");

  io::stdin().read_line(&mut scale_input)
    .expect("Failed to read line");

  let scale = scale_input.trim();
  if scale != CELSIUS_SCALE && scale != FAHRENHEIT_SCALE {
    println!("Scale must be F or C!");
    return;
  }

  let converted_value: f64 = convert(temp_degrees, scale);
  println!("Your temperature is {} degrees {}", converted_value, scale);
}
