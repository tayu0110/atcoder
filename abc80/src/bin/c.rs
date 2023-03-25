use proconio::*;

fn main() {
    input! {n: usize, f: [[u16; 10]; n], p: [[i32; 11]; n]}
    let f = f
        .into_iter()
        .map(|v| v.into_iter().fold(0, |s, v| v | (s << 1)))
        .collect::<Vec<_>>();

    let mut res = vec![0; 1 << 10];
    for i in 1..1 << 10 {
        for j in 0..n {
            let k = i & f[j];
            res[i as usize] += p[j][k.count_ones() as usize];
        }
    }

    println!("{}", res.into_iter().skip(1).max().unwrap())
}
