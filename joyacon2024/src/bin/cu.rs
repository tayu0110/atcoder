use proconio::*;

fn main() {
    input! {n: usize}

    println!(
        "{}",
        (1..=n).filter(|i| i.to_string().len() % 2 == 1).count()
    )
}
