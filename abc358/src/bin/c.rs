use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: [marker::Bytes; n]}

    let mut bits = vec![];
    for s in s {
        bits.push(
            s.into_iter()
                .fold(0, |s, v| (s << 1) | (v == b'o') as usize),
        );
    }

    let mut res = u32::MAX;
    for i in 0usize..1 << n {
        let mut r = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                r |= bits[j];
            }
        }

        if r == (1 << m) - 1 {
            res = res.min(i.count_ones());
        }
    }

    println!("{res}")
}
