use std::io;
fn main() {
  let mut list_of_ints = String::new();

  // number of ints, we agnore this
  io::stdin().read_line(&mut list_of_ints).ok().expect("read error");
  list_of_ints = "".to_string();
  // list of ints
  io::stdin().read_line(&mut list_of_ints).ok().expect("read error");

  let mut sum = 0;

  let mut iter = list_of_ints.split_whitespace();

  let mut ind_word = iter.next();

  loop {
    match ind_word {
      Some(w) => {
        let num : i32 = w.trim().parse().ok().expect("parse error");
        sum = sum + num;
      },
      None => break,
    }
    ind_word = iter.next();
  }

  println!("{}", sum);
}
