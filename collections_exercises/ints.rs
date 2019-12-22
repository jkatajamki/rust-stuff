use std::collections::HashMap;

fn get_average(v: &Vec<i32>) -> f64 {
  let mut sum = 0.0;

  for num in v {
    sum += *num as f64;
  }

  (sum / v.len() as f64).into()
}

fn get_median(v: &mut Vec<i32>) -> f64 {
  v.sort();
  let length = v.len();
  let is_even = length % 2 == 0;
  let middle_index = length / 2;

  if is_even {
    *v.get(middle_index).unwrap() as f64
  } else {
    let first_value = v.get(middle_index - 1).unwrap();
    let second_value = v.get(middle_index).unwrap();
    let middle_values = &vec![*first_value, *second_value];
    get_average(middle_values)
  }
}

fn get_mode(v: &mut Vec<i32>) -> i32 {
  let mut map = HashMap::new();

  for num in v {
    let count = map.entry(num).or_insert(0);
    *count += 1;
  }

  let mut mode = 0;

  for (_, value) in map {
    if value > mode {
      mode = value;
    }
  }

  mode
}

fn main() {
  let mut v = vec![5, 5, 4, 2, 3, 4, 5, 3];

  let avg = get_average(&v);
  println!("Average: {}", avg);

  let median = get_median(&mut v);

  println!("Median: {}", median);

  let mode = get_mode(&mut v);

  println!("Mode: {}", mode);
}
