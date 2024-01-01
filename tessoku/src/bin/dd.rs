use proconio::*;

fn main() {
    input! {n: u64}

    println!(
        "{}",
        n / 3 + n / 5 + n / 7 + n / 105 - n / 15 - n / 35 - n / 21
    );
}
