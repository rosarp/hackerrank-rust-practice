use std::io;

fn main() {
    let mut two_d_arr = [[0; 6]; 6];
    for i in 0..6 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).ok().expect("read error");
        let mut iter = line.trim().split_whitespace();
        for j in 0..6 {
            two_d_arr[i][j] = iter.next().unwrap().parse::<i32>().ok().expect("parse error");
        }
    }
    let mut max_sum = -9*7;
    for i in 0..4 {
        for j in 0..4 {
            let curr_sum = two_d_arr[i][j] + two_d_arr[i][j+1] + two_d_arr[i][j+2]
                                        + two_d_arr[i+1][j+1]
                        + two_d_arr[i+2][j] + two_d_arr[i+2][j+1] + two_d_arr[i+2][j+2];
            if curr_sum >= max_sum {
                max_sum = curr_sum;
            }
        }
    }
    println!("{}", max_sum);
}
