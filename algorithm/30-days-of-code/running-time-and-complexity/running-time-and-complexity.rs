use std::io;

fn is_prime(x : i64) -> bool {
    if x == 1 {
        return false;
    }
    let mut flag : bool = true;
    for i in 2..x {
        if x % i == 0 {
            flag = false;
            break;
        }
        if i*i > x {
            break;
        }
    }
    flag
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok().expect("read error");
    let n = n.trim().parse::<usize>().ok().expect("parse error");
    for _ in 0..n {
        let mut x = String::new();
        io::stdin().read_line(&mut x).ok().expect("read error");
        let x = x.trim().parse::<i64>().ok().expect("parse error");
        match is_prime(x) {
            true => println!("Prime"),
            false => println!("Not prime"),
        }
    }
}
