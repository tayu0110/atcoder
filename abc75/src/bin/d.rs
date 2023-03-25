use proconio::*;

fn main() {
    input! {n: usize, k: usize, p: [(i64, i64); n]}

    let mut res = std::i64::MAX;
    for i in 0..n {
        for j in i + 1..n {
            let (_, y1) = p[i];
            let (_, y2) = p[j];
            let (ymin, ymax) = (y1.min(y2), y1.max(y2));

            let mut t = vec![];
            for k in 0..n {
                if ymin <= p[k].1 && p[k].1 <= ymax {
                    t.push(p[k]);
                }
            }

            if t.len() < k {
                continue;
            }

            t.sort();
            for v in t.windows(k) {
                let x1 = v[0].0;
                let x2 = v.last().unwrap().0;
                res = res.min((ymax - ymin) * (x2 - x1));
            }
        }
    }

    println!("{}", res)
}
