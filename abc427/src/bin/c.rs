use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut ret = u32::MAX;
    for i in 0..1 << n {
        let mut cnt = 0;
        for (u, v) in e.iter().map(|(u, v)| (u - 1, v - 1)) {
            let bu = (i >> u) & 1;
            let bv = (i >> v) & 1;
            if bu == bv {
                cnt += 1;
            }
        }

        ret = ret.min(cnt);
    }

    println!("{ret}")
}
