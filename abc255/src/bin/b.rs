use proconio::input;

fn main() {
    input! {n: usize, k: usize, a: [usize; k], p: [(i64, i64); n]};

    let a = a.iter().map(|x| *x - 1).collect::<Vec<usize>>();

    let mut l = 0;
    let mut r = 111222333444555666i64;
    while r-l > 1 {
        let m = (r+l) / 2;
        let mut ok = vec![false; n];
        for v in &a {
            let (x, y) = p[*v];
            for w in 0..n {
                if ok[w] {
                    continue;
                }
                let (nx, ny) = p[w];
                if (nx - x) * (nx - x) + (ny - y) * (ny - y) <= m {
                    ok[w] = true;
                }
            }
        }

        let res = ok.iter().fold(true, |res, x| res & *x);
        if res {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", (r as f64).sqrt());
}