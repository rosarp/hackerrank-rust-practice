use std::io;

fn main() {
    let mut time = String::new();
    io::stdin().read_line(&mut time).ok().expect("read error");
    let time_vec = time.trim().split(':').collect::<Vec<&str>>();
    match &time_vec[2][2..4]  {
        "AM" => {
            let hour = time_vec[0].trim().parse::<u32>().ok().expect("parse error");
            match hour {
                12 => println!("00:{}:{}", time_vec[1], &time_vec[2][0..2]),
                _ => println!("{}", &time[0..8]),
            }
        },
        "PM" => {
            let mut hour = time_vec[0].trim().parse::<u32>().ok().expect("parse error");
            match hour {
                12 => println!("12:{}:{}", time_vec[1], &time_vec[2][0..2]),
                _ => {
                    hour += 12;
                    println!("{}:{}:{}", hour, time_vec[1], &time_vec[2][0..2]);
                }
            }
        },
        _ => {}
    }
}
