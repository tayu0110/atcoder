use proconio::*;

fn main() {
    input! {h: f64, bmi: f64}

    println!("{}", bmi * h * h / 10000.0);
}
