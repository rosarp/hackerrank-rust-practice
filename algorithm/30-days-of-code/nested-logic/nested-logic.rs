use std::io;
use std::str::SplitWhitespace;

fn matcher(iter : &mut SplitWhitespace) -> i32 {
    match iter.next() {
        Some(x) => x.parse::<i32>().ok().expect("parse error"),
        None => panic!("Not expecting panic! But paniced!"),
    }
}

struct Date {
    dd : i32,
    mm : i32,
    yyyy : i32,
}

impl Date {
    fn new() -> Date {
        let mut date = String::new();
        io::stdin().read_line(&mut date).ok().expect("read error");
        let date = date.trim().parse::<String>().ok().expect("parse error");
        let mut iter : SplitWhitespace = date.split_whitespace();
        let dd = matcher(&mut iter);
        let mm = matcher(&mut iter);
        let yyyy = matcher(&mut iter);
        //let dd = iter.next().unwrap().parse::<i32>().ok().expect("parse error");
        //let mm = iter.next().unwrap().parse::<i32>().ok().expect("parse error");
        //let yyyy = iter.next().unwrap().parse::<i32>().ok().expect("parse error");
        Date {
            dd : dd,
            mm : mm,
            yyyy : yyyy,
        }
    }
    fn calculate_fine(&self, expected : Date) -> i32 {
        if (self.yyyy < expected.yyyy) ||
            (self.yyyy == expected.yyyy && (self.mm < expected.mm ||
                (self.mm == expected.mm && self.dd <= expected.dd))) {
            0
        } else if self.dd > expected.dd && self.mm == expected.mm && self.yyyy == expected.yyyy {
            15 * (self.dd - expected.dd)
        } else if self.mm > expected.mm && self.yyyy == expected.yyyy {
            500 * (self.mm - expected.mm)
        } else {
            10_000
        }
    }
}

fn main() {
    let returned = Date::new();
    let expected = Date::new();
    let fine = returned.calculate_fine(expected);
    println!("{}", fine);
}
