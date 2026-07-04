use convolution::convolution_mod_2_64;
use proconio::*;

fn main() {
    input! {n: usize, a: [u64; n]}
    let mut count = vec![0; (a.iter().max().unwrap() + 1) as usize];
    for &a in &a {
        count[a as usize] += 1;
    }
    let res = convolution_mod_2_64(count.clone(), count);
    let res = res
        .into_iter()
        .enumerate()
        .filter_map(|(i, r)| (r > 0).then(|| r * (i >> i.trailing_zeros()) as u64))
        .sum::<u64>();
    let a = a.into_iter().map(|a| a >> a.trailing_zeros()).sum::<u64>();
    println!("{}", (res - a) / 2 + a)
}
