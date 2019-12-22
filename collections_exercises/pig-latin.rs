use std::io;

fn pig_latinize_word(word: &str) -> String {
  let mut chars = word.chars();
  let first_char = chars.next().unwrap();
  let sans_first_char = chars.as_str();

  let lower_case = first_char
    .to_string()
    .to_lowercase();

  let common_end = String::from("ay");


  let pig_latin_word = match lower_case.as_ref() {
    "a" | "e" | "i" | "o" | "u" => format!("{}-{}", sans_first_char, common_end),
    _ => format!("{}-{}{}", sans_first_char, String::from(lower_case), common_end),
  };

  pig_latin_word
}

fn pig_latinize(v: Vec<&str>) -> String {
  let mut pig_latin_v: Vec<String> = Vec::new();

  for word in v {
    let translated_word = pig_latinize_word(word);
    pig_latin_v.push(translated_word);
  }

  pig_latin_v.join(" ")
}

fn main() {
  println!("Enter the string you want to convert:");

  let mut input_string = String::new();

  io::stdin().read_line(&mut input_string).expect("Failed to read line");

  let trimmed = input_string.trim();

  let v: Vec<&str> = trimmed.split(' ').collect();

  let translated_v = pig_latinize(v);
  println!("Translated: {:?}", translated_v);
}
