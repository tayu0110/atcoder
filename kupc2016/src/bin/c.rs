use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, d: usize}

        println!(
            "{}",
            127 * (n - 1) + if (n - 1) % 2 == 0 { d } else { d ^ 127 }
        );
    }
}
