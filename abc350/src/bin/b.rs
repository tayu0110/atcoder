use proconio::*;

fn main() {
    input! {n: usize, q: usize, t: [usize; q]}

    let mut init = vec![true; n + 1];
    for t in t {
        init[t] = !init[t];
    }

    println!("{}", init[1..].iter().map(|&f| f as usize).sum::<usize>())
}
