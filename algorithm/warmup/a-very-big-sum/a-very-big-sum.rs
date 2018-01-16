use std::io;

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok().expect("read error!");
  let cnt = line.trim().parse::<usize>().unwrap();
  line.clear();
  io::stdin().read_line(&mut line).ok().expect("read error!");
  let v: Vec<&str> = line.trim().split(' ').collect();
  let mut sum: u64 = 0;
  for i in 0..cnt {
    sum += v[i].parse::<u64>().unwrap();
  }
  println!("{}", sum);
}
