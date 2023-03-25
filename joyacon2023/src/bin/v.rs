use proconio::input;

fn main() {
    input! {a: usize, b: usize, c: usize, x: usize}

    if x <= a {
        println!("1")
    } else if a < x && x <= b {
        println!("{}", c as f64 / (b - a) as f64);
    } else {
        println!("0")
    }
}
