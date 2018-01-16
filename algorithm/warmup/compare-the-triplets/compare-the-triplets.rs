use std::io;

fn main() {
  let mut line1 = String::new();
  io::stdin().read_line(&mut line1).ok().expect("read error");
  let av: Vec<&str> = line1.trim().split(' ').collect();
  let mut line2 = String::new();
  io::stdin().read_line(&mut line2).ok().expect("read error");
  let bv: Vec<&str> = line2.trim().split(' ').collect();
  let mut a_pt : i32 = 0;
  let mut b_pt : i32 = 0;
  for i in 0..3 {
    let a = av[i].parse::<i32>().unwrap();
    let b = bv[i].parse::<i32>().unwrap();
    if a > b {
      a_pt += 1;
    } else if a < b {
      b_pt += 1;
    }
  }
  println!("{} {}", a_pt, b_pt);
}
