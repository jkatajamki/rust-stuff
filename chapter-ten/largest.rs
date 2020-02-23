fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn get_largest_reference<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list.iter() {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn get_largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
  let mut largest = list[0].clone();

  for item in list.iter() {
    if item > &largest {
      largest = item.clone();
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = get_largest(&number_list);
  println!("Largest number is: {}", result);

  let result_reference = get_largest_reference(&number_list);
  println!("Reference to largest number is: {}", result_reference);

  let result_clone = get_largest_clone(&number_list);
  println!("Cloned largest number is: {}", result_clone);
}
