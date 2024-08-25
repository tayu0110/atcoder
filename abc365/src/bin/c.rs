use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    const MAX: usize = 100000000000000;
    let (mut l, mut r) = (0, MAX);
    while r - l > 1 {
        let x = (r + l) / 2;
        let sum = a.iter().map(|&a| a.min(x)).sum::<usize>();
        if sum <= m {
            l = x;
        } else {
            r = x;
        }
    }

    if &l >= a.iter().max().unwrap() {
        println!("infinite")
    } else {
        println!("{l}")
    }
}
