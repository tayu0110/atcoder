use proconio::*;

fn main() {
    input! {n: usize, mut p: [(usize, usize); n]}
    p.sort_unstable_by_key(|v| v.1);

    let mut res = 0;
    for i in 0..n {
        let (_, x) = p[i];
        let mut sum = 0;
        for j in 0..n {
            let (nw, nx) = p[j];
            if (x <= nx && nx < x + 9) || (x <= nx + 24 && nx + 24 < x + 9) {
                sum += nw;
            }
        }

        res = res.max(sum);
    }

    println!("{res}");
}
