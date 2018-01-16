use std::io;

fn split_odd_even_chars(line: &String) -> String {
    let mut review = String::new();
    let mut odd_review = String::new();
    let iter = line.trim().chars();
    for (i, v) in iter.enumerate() {
        if i % 2 == 0 {
            review.push(v);
        } else {
            odd_review.push(v);
        }
    }

    review.push(' ');
    review.push_str(&odd_review);
    review
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok().expect("read error");
    let n: usize = n.trim().parse().ok().expect("parse error");

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).ok().expect("read error");
        println!("{}", split_odd_even_chars(&line));
    }
}
