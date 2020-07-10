use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, K, V>
  where T: Fn(K) -> V
{
  calculation: T,
  values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
  where T: Fn(K) -> V,
  K: Hash + Eq + Clone + Copy,
  V: Copy
{
  fn new(calculation: T) -> Cacher<T, K, V> {
    let values_hash_map = HashMap::new();

    Cacher {
      calculation,
      values: values_hash_map,
    }
  }

  fn value(&mut self, arg: K) -> V {
    match self.values.get(&arg) {
      Some(v) => *v,
      None => {
        let v = (self.calculation)(arg);
        self.values.insert(arg, v);
        v
      }
    }
  }
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut expensive_result = Cacher::new(|num| {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!("Do {} backflips!", expensive_result.value(intensity));
    println!("Then, do {} frontflips!", expensive_result.value(intensity));
  } else {
    if random_number == 3 {
      println!("Take a break today! Don't over-extend yourself! Go get yourself a cookie, you've deserved it! Would you like a hug!?")
    } else {
      println!("Do bench press for {} sets of five!", expensive_result.value(intensity))
    }
  }
}

fn main() {
  let simulated_user_specified_value = 50;
  let simulated_random_number = 3;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
  }
}
