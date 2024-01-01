use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n]}

    a.sort();
    a.reverse();

    let (a, b) = a.split_at_mut(m);
    a.reverse();

    a[..b.len()]
        .iter_mut()
        .zip(b.into_iter())
        .for_each(|(l, r)| *l += *r);

    println!("{}", a.into_iter().map(|l| *l * *l).sum::<usize>())
}
