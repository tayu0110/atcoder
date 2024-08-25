use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut b = vec![];
    for _ in 0..m {
        input! {k: usize, s: [u8; k]}
        let mut t = 0u16;
        for s in s {
            t |= 1 << (s - 1);
        }
        b.push(t);
    }

    input! {p: [u32; m]}

    let mut res = 0;
    for i in 0..1 << n {
        if b.iter()
            .enumerate()
            .all(|(j, &b)| (b & i).count_ones() % 2 == p[j])
        {
            res += 1;
        }
    }

    println!("{res}")
}
