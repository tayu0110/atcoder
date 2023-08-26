use proconio::*;

fn main() {
    input! {n: usize, m: usize, x: usize}

    let mut c = vec![];
    let mut a = vec![];
    for _ in 0..n {
        input! {nc: usize, na: [usize; m]}
        c.push(nc);
        a.push(na);
    }

    let mut res = std::usize::MAX;
    for i in 0..1 << n {
        let mut sum = vec![0; m];
        let mut val = 0;
        for j in 0..n {
            if i & (1 << j) != 0 {
                val += c[j];
                for (i, &a) in a[j].iter().enumerate() {
                    sum[i] += a;
                }
            }
        }

        if sum.iter().all(|&s| s >= x) {
            res = res.min(val);
        }
    }

    if res == std::usize::MAX {
        println!("-1")
    } else {
        println!("{}", res)
    }
}
