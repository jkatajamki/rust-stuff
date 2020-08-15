use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("ho"),
      String::from("let's"),
      String::from("go"),
    ];

    for val in vals {
      tx.send(val).unwrap();

      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Knowledge divined from beyond the veil: {}", received);
  }
}
