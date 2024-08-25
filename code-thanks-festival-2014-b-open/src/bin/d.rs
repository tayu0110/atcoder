use proconio::*;

fn main() {
    input! {n: usize, t: usize, a: [usize; n]}

    let mut memo = vec![0; t + 1];
    for a in a {
        for i in (1..).take_while(|i| a * i <= t) {
            memo[a * i] += 1;
        }
    }

    println!("{}", memo.iter().max().unwrap())
}
