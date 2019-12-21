fn get_average(v: &Vec<i32>) -> f64 {
  let mut sum = 0.0;

  for num in v {
    sum += *num as f64;
  }

  (sum / v.len() as f64).into()
}

fn get_median(v: &mut Vec<i32>) -> i32 {
  let mut sorted = v.sort();


  3
}

fn main() {
  let mut v = vec![5, 4, 2, 3, 4, 5];

  let avg = get_average(&v);
  println!("Average: {}", avg);

  let median = get_median(&mut v);

  println!("Median: {}", median);

}
