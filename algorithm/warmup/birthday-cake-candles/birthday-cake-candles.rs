use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok().expect("read error");

    let mut src = String::new();
    io::stdin().read_line(&mut src).ok().expect("read error");
    let num_list = src.trim().split_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
    let max_num : &u64 = num_list.iter().max().unwrap();
    let iter = num_list.iter().filter(|x| *x == max_num);
    println!("{}", iter.count());
}
