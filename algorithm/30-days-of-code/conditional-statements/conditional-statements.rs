use std::io;

fn main() {
  let mut line = String::new();
  io::stdin().read_line(&mut line).ok().expect("read error");

  match line.trim().parse::<i32>().unwrap() {
    n if (n % 2 != 0) || (n % 2 == 0 && n >= 6 && n <= 20) => {
      println!("Weird");
    },
    n if (n % 2 == 0) && ((n >= 2 && n <= 5) || n > 20) => {
      println!("Not Weird");
    },
    _ => {
      println!("Weird");
    }
  }
}
