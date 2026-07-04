use proconio::*;

fn main() {
    input! {h: usize, w: usize, d: usize, t: [marker::Bytes; h]}

    let mut pos = vec![];
    for i in 0..h {
        for j in 0..w {
            if t[i][j] == b'.' {
                pos.push((i, j));
            }
        }
    }

    let mut res = 0;
    for (i, &(r, c)) in pos.iter().enumerate() {
        for &(nr, nc) in pos.iter().skip(i + 1) {
            let mut sum = 0;
            for &(i, j) in &pos {
                let k = i.abs_diff(r) + j.abs_diff(c);
                let l = i.abs_diff(nr) + j.abs_diff(nc);
                if k.min(l) <= d {
                    sum += 1;
                }
            }
            res = res.max(sum);
        }
    }

    println!("{res}")
}
