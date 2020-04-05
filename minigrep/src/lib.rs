use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::env;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments")
    }

    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config { query, filename, case_sensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let mut f = File::open(config.filename)?;

  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
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

  #[test]
  fn case_sensitive() {
    let query = "able";
    let contents = "\
Writing a poem

In seventeen syllables

Is very diff
(... are you Able?)
";

    assert_eq!(
      vec!["In seventeen syllables"],
      search(query, contents)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "pOeM";
    let contents = "\
Writing a poem

In seventeen syllables

Is very diff
(... Best Poem Ever.)
";

    assert_eq!(
      vec!["Writing a poem", "(... Best Poem Ever.)"],
      search_case_insensitive(query, contents)
    );
  }
}
