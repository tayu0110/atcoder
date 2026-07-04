use proconio::*;

fn main() {
    input! {n: usize}

    let mut bits = vec![];
    for _ in 0..n {
        input! {c: usize, a: [usize; c]}
        bits.push(
            a.into_iter()
                .filter(|&a| a % 2 == 0)
                .fold(0usize, |s, v| s | (1 << (v / 2))),
        );
    }

    let mut res = 0;
    for i in (3u32..1 << n).filter(|&i| i.count_ones() >= 2) {
        let bit = (0..n)
            .filter(|&j| i & (1 << j) != 0)
            .fold(usize::MAX, |s, v| s & bits[v]);
        if bit == 0 {
            res += 1;
        }
    }

    println!("{}", res)
}
