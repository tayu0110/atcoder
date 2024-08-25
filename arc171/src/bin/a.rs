use proconio::*;

fn main() {
    input! {t: usize}

    let mut res = vec![];
    for _ in 0..t {
        input! {mut n: usize, a: usize, b: usize}

        if a > n {
            res.push("No");
            continue;
        }

        let (mut h, mut w) = (n, n);
        w -= a;
        let hn = n / 2;
        if a <= hn {
            h -= hn;
        } else {
            h -= a;
        }

        if b <= h * w {
            res.push("Yes")
        } else {
            res.push("No");
        }
    }

    for s in res {
        println!("{s}");
    }
}
