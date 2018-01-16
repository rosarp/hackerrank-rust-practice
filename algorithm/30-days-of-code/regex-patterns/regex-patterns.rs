use std::io;

fn is_gmail(s : String) -> bool {
    let mut iter = s.split("@");
    iter.next();
    let domain = iter.next().unwrap().parse::<String>().ok().expect("parse error");
    let expected_domain = String::from("gmail.com");
    if expected_domain == domain {
        true
    } else {
        false
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok().expect("read error");
    let n = n.trim().parse::<usize>().ok().expect("parse error");
    let mut list = vec![String::from(""); 0];
    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).ok().expect("read error");
        let s = s.trim().parse::<String>().ok().expect("parse error");
        let mut iter = s.split_whitespace();
        let name = iter.next().unwrap().parse::<String>().ok().expect("parse error");
        let email = iter.next().unwrap().parse::<String>().ok().expect("parse error");
        match is_gmail(email) {
            true => {list.push(name)},
            false => {},
        }
    }
    list.sort();
    for i in 0..list.len() {
        println!("{}", list[i]);
    }
}
