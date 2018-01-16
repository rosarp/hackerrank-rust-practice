use std::io;

fn read_input() -> (i32, i32) {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");
    let s = s.trim().parse::<String>().ok().expect("parse error");
    let mut iter = s.split_whitespace();
    let n = iter.next().unwrap().parse::<i32>().ok().expect("parse error");
    let k = iter.next().unwrap().parse::<i32>().ok().expect("parse error");
    (n, k)
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");
    let sn = s.trim().parse::<i32>().ok().expect("parse error");
    for _ in 0..sn {
        let (n, k) = read_input();
        let mut mx = 0;
        for a in 1..n {
            for b in a+1..n+1 {
                let ab = a & b;
                if (ab >= mx) && (ab < k) {
                    mx = a & b;
                }
            }
        }
        println!("{}", mx);
    }
}
