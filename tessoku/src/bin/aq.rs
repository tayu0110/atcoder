use proconio::*;

fn main() {
    input! {n: usize, l: usize, p: [(usize, char); n]}

    println!(
        "{}",
        p.into_iter()
            .map(|(a, b)| {
                if b == 'E' {
                    l - a
                } else {
                    a
                }
            })
            .max()
            .unwrap()
    )
}
