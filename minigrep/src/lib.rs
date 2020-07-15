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
  pub fn new<T>(mut args: T) -> Result<Config, &'static str>
    where T: Iterator<Item = String>
  {
    // First arg is name of the program, which is ignored
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };

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
  contents.lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query_lower = query.to_lowercase();

  contents.lines()
    .filter(|line| line.to_lowercase().contains(&query_lower))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_initializes_config() {
    let arg_strs = ["programname", "first", "second"];
    let args = arg_strs.iter().map(|s| s.to_string());

    let config = Config::new(args).unwrap();

    assert_eq!(arg_strs[1], config.query);
    assert_eq!(arg_strs[2], config.filename);
  }

  #[test]
  fn it_fails_to_init_config() {
    let args = ["programname", "first"].iter().map(|s| s.to_string());

    let result = Config::new(args).is_err();

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
