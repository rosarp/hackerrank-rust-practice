use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok().expect("read error");
    let n = line.trim().parse::<usize>().ok().expect("parse error");
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok().expect("read error");
    let mut arr: [i32; 1000] = [0; 1000];
    let iter = line.trim().split_whitespace();
    for (i, v) in iter.enumerate() {
        if i > n { break; }
        arr[i] = v.parse::<i32>().ok().expect("parse error");
    }
    for i in (0..n).rev() {
        print!("{} ", arr[i]);
    }
}
