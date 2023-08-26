use proconio::*;

fn main() {
    input! {x: i64}

    let f = |a: i64, b: i64| -> Option<i64> {
        let mut overflowed = false;
        let mut na = vec![1i64];
        let mut nb = vec![1i64];
        for i in 0..4 {
            let (t, f) = na[i].overflowing_mul(a);
            overflowed |= f;
            na.push(t);
            let (t, f) = nb[i].overflowing_mul(b);
            nb.push(t);
            overflowed |= f;
        }
        let mut res = 0;
        for i in 0..=4 {
            let (t, f) = na[i].overflowing_mul(nb[4 - i]);
            overflowed |= f;
            let (t, f) = t.overflowing_add(res);
            overflowed |= f;
            res = t;
        }
        if overflowed {
            None
        } else {
            Some(res)
        }
    };
    for i in (1..=x).take_while(|&i| i * i <= x).filter(|&i| x % i == 0) {
        for j in vec![i, x / i] {
            for d in vec![1, -1] {
                let (mut a, mut b) = (j, 0);
                while let Some(res) = f(a, b) {
                    if res == x / j {
                        println!("{} {}", a, b);
                        return;
                    }
                    a += d;
                    b += d;
                }
                let (mut a, mut b) = (0, -j);
                while let Some(res) = f(a, b) {
                    if res == x / j {
                        println!("{} {}", a, b);
                        return;
                    }
                    a += d;
                    b += d;
                }
            }
        }
    }
}
