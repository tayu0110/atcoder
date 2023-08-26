use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut res = std::usize::MAX;
    for &(a, _) in &p {
        for &(_, b) in &p {
            let mut sum = 0;
            for &(l, r) in &p {
                let l = if l < a {
                    sum += a - l;
                    l
                } else {
                    a
                };
                let r = if b < r {
                    sum += r - b;
                    r
                } else {
                    b
                };
                sum += r - l;
            }

            res = res.min(sum);
        }
    }

    println!("{}", res)
}
