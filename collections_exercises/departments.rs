use std::io;
use std::collections::HashMap;

fn main() {
  let mut departments: HashMap<String, Vec<String>> = HashMap::new();

  loop {
    println!("Enter command:");
    println!("Example: Add <name> to <department>");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line!");
    let command = input.trim();
    let v: Vec<&str> = command.split(' ').collect();

    let input_name = String::from(v[1]);
    let input_department = String::from(v[3]);

    let matching_name = departments.get(&input_department);
    match matching_name {
      Some(x) => {
        let mut new_names = x.to_vec();
        new_names.push(input_name);

        departments.insert(input_department, new_names);
      },
      None => {
        let new_names = vec![String::from(&input_name)];

        departments.insert(input_department, new_names);
      },
    }

    println!("Departments: {:?}", departments);
  }
}
