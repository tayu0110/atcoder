use proconio::*;

fn main() {
    input! {_n: usize, l: usize, r: usize, s: marker::Bytes}

    let mut t = vec![vec![]; 128];
    let mut ret = 0;
    for (i, b) in s.into_iter().enumerate() {
        let nl = t[b as usize].partition_point(|&t| t + r < i);
        let nr = t[b as usize].partition_point(|&t| t + l <= i);
        ret += nr - nl;
        t[b as usize].push(i);
    }

    println!("{}", ret);
}
