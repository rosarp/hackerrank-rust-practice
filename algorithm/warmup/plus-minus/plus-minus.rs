use std::io;

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok().expect("read error");
  let arr_size = line.trim().parse::<usize>().ok().expect("parse error");
  line.clear();
  io::stdin().read_line(&mut line).ok().expect("read error");
  let (mut p, mut n, mut z) : (f32, f32, f32) = (0.0, 0.0, 0.0);
  let mut iter = line.trim().split_whitespace();
  for _ in 0..arr_size {
    match iter.next() {
      Some(s) if s.parse::<i32>().unwrap() > 0 => p += 1.0,
      Some(s) if s.parse::<i32>().unwrap() < 0 => n += 1.0,
      Some(s) if s.parse::<i32>().unwrap() == 0 => z += 1.0,
      _ => println!("parsing error"),
    }
  }
  println!("{:.6}", p/(arr_size as f32));
  println!("{:.6}", n/(arr_size as f32));
  println!("{:.6}", z/(arr_size as f32));
}
