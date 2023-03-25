use proconio::input;

fn main() {
    input! {a: i32, b: i32}

    let d = (a as f64).hypot(b as f64);

    println!("{} {}", (a as f64) / d, (b as f64) / d);
}
