use proconio::*;

fn main() {
    input! {n: usize}

    if n <= 3 || n == 6 {
        println!("-1")
    } else if n == 4 {
        println!("2 3 2 1")
    } else if n == 5 {
        println!("3 2 3 1 1")
    } else if n == 7 {
        println!("4 3 2 2 1 1 1")
    } else {
        println!("{} 3 2", n - 3);
        for _ in 4..=n - 4 {
            print!(" 1");
        }
        println!(" 2 1 1 1");
    }
}
