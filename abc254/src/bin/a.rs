use proconio::input;

fn main() {
    input! {n: usize};

    if n % 100 < 10 {
        print!("0")
    }
    println!("{}", n % 100);
}