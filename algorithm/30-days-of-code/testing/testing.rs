fn main() {
    let mut n = 15;
    let expected_results = [-1, 1, -1, 1, -1];
    println!("5");
    for t in 0..5 {
        let nb2 = n/2;
        let k = nb2 + expected_results[t];
        let km = nb2 + expected_results[t] + expected_results[t];
        println!("{:?} {}", n, k);
        print!("{} {} {} ", -1, 0, 1);
        for ai in 3..km {
            print!("{:?} ", ai*expected_results[t]);
        }
        for ai in km..n {
            print!("{:?}", ai*expected_results[t]*-1);
            if ai+1 != n {
                print!(" ");
            }
        }
        println!("");
        n -= 1;
    }
}
