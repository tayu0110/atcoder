use proconio::*;

fn main() {
    input! {l: i64, r: i64, d: i64, u: i64}

    let mut ret = 0;
    for i in l..=r {
        let u = std::cmp::min_by_key(u, i, |k| k.abs());
        let d = std::cmp::max_by_key(d, i, |k| k.abs());
        if i % 2 == 0 {
            ret += (u - d + 1).max(0);
        } else {
        }
    }

    println!("{ret}")
}
