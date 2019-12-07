const NUM_OF_ELEMENTS: usize = 20;

fn main() {
  // calculate fib sequence for NUM_OF_ELEMENTS elements
  let mut fib_seq = vec![1, 1];

  for x in 2..NUM_OF_ELEMENTS {
    let prev_i = fib_seq[x - 1];
    let prev_of_prev_i = fib_seq[x - 2];
    let next_num = prev_i + prev_of_prev_i;
    fib_seq.push(next_num);
  }

  println!("Fib sequence for {} elements calculated", NUM_OF_ELEMENTS);
  for n in fib_seq.iter() {
    print!("{} ", n);
  }
}
