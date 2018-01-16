use std::io;

struct MyArray {
    arr : Vec<i32>,
    num_swaps : i32,
}

impl MyArray {
    #[inline]
    fn swap_arr(&mut self, x : usize, y : usize) {
        let temp = self.arr[x];
        self.arr[x] = self.arr[y];
        self.arr[y] = temp;
        self.num_swaps += 1;
    }
    fn bubble_sort(&mut self) {
        let n = self.arr.len();
        let mut temp_num_swaps : i32 = 0;
        for _ in 0..n {
            for j in 0..n-1 {
                if self.arr[j] > self.arr[j+1] {
                    self.swap_arr(j as usize, j+1 as usize);
                    temp_num_swaps += 1;
                }
            }
            if temp_num_swaps == 0 {
                break;
            }
        }
    }
    fn print_array(&self) {
        println!("Array is sorted in {} swaps.", self.num_swaps);
        println!("First Element: {}", self.arr[0]);
        let arr_len = self.arr.len();
        println!("Last Element: {}", self.arr[arr_len-1]);
    }
}

#[inline]
fn fill_array(my_arr: &mut MyArray) {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok().expect("read error");
    let iter = n.trim().split_whitespace();
    for (i,v) in iter.enumerate() {
        my_arr.arr[i] = v.parse::<i32>().ok().expect("parse error");
    }
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok().expect("read error");
    let n = n.trim().parse().ok().expect("parse error");
    let mut my_arr = MyArray { arr : vec![0; n], num_swaps : 0 };
    fill_array(&mut my_arr);
    my_arr.bubble_sort();
    my_arr.print_array();
}
