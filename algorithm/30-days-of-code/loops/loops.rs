use std::io;

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok().expect("read error");
  let n = line.trim().parse::<i32>().ok().expect("parse error");
  for i in 0..10 {
    println!("{} x {} = {}", n, i+1, n*(i+1));
  }
}
