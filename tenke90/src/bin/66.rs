use proconio::input;

fn main() {
    input! {n: usize, p: [(usize, usize); n]};

    let mut res = 0f64;
    for (i, (l, r)) in p.iter().enumerate() {
        let (l, r) = (*l, *r);
        for j in 0..i {
            let (pl, pr) = p[j];
            let mut cnt = 0;
            for k in l..r+1 {
                for l in pl..pr+1 {
                    if l > k {
                        cnt += 1;
                    }
                }
            }

            res += cnt as f64 / ((r-l+1) * (pr-pl+1)) as f64;
        }
    }

    println!("{}", res);
}