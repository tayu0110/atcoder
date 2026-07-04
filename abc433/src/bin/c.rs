use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut rle = vec![];
    for b in s {
        match rle.last_mut() {
            Some((p, cnt)) if *p == b => *cnt += 1,
            _ => rle.push((b, 1)),
        }
    }

    let mut ret = 0;
    for v in rle.windows(2) {
        let (lp, lc) = v[0];
        let (rp, rc) = v[1];
        if lp + 1 == rp {
            ret += lc.min(rc);
        }
    }

    println!("{}", ret)
}
