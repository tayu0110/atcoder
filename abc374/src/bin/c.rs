use proconio::*;

fn main() {
    input! {n: usize, k: [usize; n]}

    let mut res = usize::MAX;
    for i in 0..1 << n {
        let a = (0..n)
            .filter(|&j| i & (1 << j) != 0)
            .map(|i| k[i])
            .sum::<usize>();
        let b = k.iter().sum::<usize>() - a;
        res = res.min(a.max(b));
    }
    println!("{res}")
}
