use std::io;

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok().expect("read error");
  let n = line.trim().parse::<usize>().ok().expect("parse error");
  let mut pri_sum : i32 = 0;
  let mut sec_sum : i32 = 0;

  for i in 0..n {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok().expect("read error");
    let temp_v : Vec<&str> = line.trim().split(' ').collect();
    pri_sum += temp_v[i].parse::<i32>().ok().expect("parse error");
    sec_sum += temp_v[n-1-i].parse::<i32>().ok().expect("parse error");
  }

  println!("{}", (pri_sum - sec_sum).abs());
}
