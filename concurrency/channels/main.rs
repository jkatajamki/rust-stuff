use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
  let (tx, rx) = mpsc::channel();

  let tx1 = mpsc::Sender::clone(&tx);

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("ho"),
      String::from("let's"),
      String::from("go"),
    ];

    for val in vals {
      tx1.send(val).unwrap();

      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("They're forming in a straight line"),
      String::from("They're going through a tight wind"),
      String::from("The kids are losing their minds"),
      String::from("the blitzkrieg bop"),
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
