use std::io;

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok().expect("read error");
  let n = line.trim().parse::<usize>().ok().expect("parse error");
  for i in 1..n+1 {
    for _ in 0..n-i {
      print!("{}", " ")
    }
    for _ in 0..i {
      print!("{}", "#")
    }
    println!("");
  }
}
