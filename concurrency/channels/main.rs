use std::thread;
use std::sync::mpsc;

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi ho");

    tx.send(val).unwrap();
  });

  let received = rx.recv().unwrap();
  println!("Knowledge divined from beyond the veil: {}", received);
}
