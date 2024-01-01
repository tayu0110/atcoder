use proconio::*;

fn main() {
    input! {n: usize, w: [usize; n]}

    let mut res = usize::MAX;
    for i in 1..n {
        res = res.min(
            w[..i]
                .iter()
                .sum::<usize>()
                .abs_diff(w[i..].iter().sum::<usize>()),
        );
    }

    println!("{res}")
}
