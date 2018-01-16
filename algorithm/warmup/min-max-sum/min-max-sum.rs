use std::io;

fn main() {
    let mut src = String::new();
    io::stdin().read_line(&mut src).ok().expect("read error");
    let num_list : Vec<u64> = src.trim().split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
    let sum : u64 = num_list.iter().sum();
    let min_sum : u64 = sum - num_list.iter().max().unwrap();
    let max_sum : u64 = sum - num_list.iter().min().unwrap();
    println!("{} {}", min_sum, max_sum);
}
