use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments")
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let mut f = File::open(config.filename)?;

  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  println!("With text: \n{}", contents);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_initializes_config() {
    let args = vec!["programname".to_string(), "first".into(), "second".into()];

    let config = Config::new(&args).unwrap();

    assert_eq!(&args[1], &config.query);
    assert_eq!(&args[2], &config.filename);
  }

  #[test]
  fn it_fails_to_init_config() {
    let args = vec!["programname".to_string(), "first".into()];

    let result = Config::new(&args).is_err();

    let expected = true;

    assert_eq!(result, expected);
  }
}
