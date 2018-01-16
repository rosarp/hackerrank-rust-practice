use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::str::SplitWhitespace;

fn read_line() -> String {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("Could not read stdin!");
  return input;
}

fn main() {
    let line = read_line();
    let n = line.trim().parse::<usize>().ok().expect("parse error");
    let mut phonebook : HashMap<String, String> = HashMap::new();
    // read phonebook
    for _ in 0..n {
        let line = read_line();
        let mut entry_iter: SplitWhitespace = line.trim().split_whitespace();
        phonebook.insert(entry_iter.next().unwrap().to_string(), entry_iter.next().unwrap().to_string());
    }
    // read queries
    let reader = io::stdin();
    for line in reader.lock().lines() {
        let query : String = line.unwrap();
        match phonebook.get::<str>(&query) {
            Some(phone_number) => println!("{}={}", query, phone_number),
            None => println!("Not found"),
        }
    }
}
