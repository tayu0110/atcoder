use proconio::*;

fn main() {
    input! {l: u128, r: u128}

    let mut now = l;
    let mut res = vec![];
    while now < r {
        let mut tr = if now > 0 { now.trailing_zeros() } else { 61 };
        let mut k = now >> tr;
        while (k + 1) << tr > r {
            tr -= 1;
            k <<= 1;
        }

        res.push((now, (k + 1) << tr));
        now = (k + 1) << tr;
    }

    println!("{}", res.len());
    for (l, r) in res {
        println!("{l} {r}")
    }
}
