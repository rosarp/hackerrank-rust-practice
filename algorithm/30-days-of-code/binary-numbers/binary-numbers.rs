use std::io;

fn find_upper_bound(n : i32) -> i32 {
    let mut i : i32 = 0;
    loop {
        if n < (2<<i) {
            break;
        }
        i += 1;
    }
    i
}

fn find_max_ones(mut n : i32) -> i32 {
    if n == 1 {
        return 1;
    }
    let mut max_ones : i32 = 1;
    let mut consecutive_ones : i32 = 1;
    let mut was_last_one : bool = false;
    let upper_bound : i32 = find_upper_bound(n);
    for i in (0..upper_bound).rev() {
        if n - (2<<i) >= 0 {
        // this is 1
            n = n - (2<<i);
            if was_last_one == true {
                consecutive_ones += 1;
            } else {
                consecutive_ones = 1;
            }
            if consecutive_ones > max_ones {
                max_ones = consecutive_ones;
            }
            was_last_one = true;
        } else {
        // this is 0
            was_last_one = false;
        }
    }
    if n == 1 && was_last_one == true {
        consecutive_ones += 1;
    }
    if consecutive_ones > max_ones {
        max_ones = consecutive_ones;
    }
    max_ones
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok().expect("read error");
    let n = line.trim().parse::<i32>().ok().expect("parse error");
    println!("{}", find_max_ones(n));
}
