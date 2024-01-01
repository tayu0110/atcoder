use proconio::*;

fn main() {
    input! {k: i64, a: i64, b: i64}

    if a >= k {
        println!("1");
        return;
    }

    let d = a - b;
    if d <= 0 {
        println!("-1");
        return;
    }

    println!("{}", 1 + ((k - a + d - 1) / d) * 2);
}
