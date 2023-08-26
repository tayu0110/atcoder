use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();
    let (a, b) = a.split_at_mut(n / 2);
    b.reverse();
    let k = a
        .into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| *a + *b)
        .collect::<Vec<_>>();
    println!("{}", k.iter().max().unwrap() - k.iter().min().unwrap())
}
