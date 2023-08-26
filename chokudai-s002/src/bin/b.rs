use proconio::input;

fn main() {
    input! {n: usize}

    for _ in 0..n {
        input! {a: usize, b: usize}
        println!("{}", a % b)
    }
}
