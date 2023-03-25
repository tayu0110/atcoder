use proconio::input;

fn main() {
    input! {k: usize}

    let mut map = std::collections::HashMap::new();
    {
        let mut i = 2;
        let mut now = k;
        while i * i <= k {
            while now % i == 0 {
                *map.entry(i).or_insert(0) += 1;
                now /= i;
            }
            i += 1;
        }

        if now != 1 {
            map.insert(now, 1);
        }
    }

    let (mut l, mut r) = (0, k);
    while r - l > 1 {
        let m = (r + l) / 2;

        let mut bad = false;
        for (&i, &v) in &map {
            let mut t = 0;
            let mut f = i;
            while f <= m {
                t += m / f;
                f *= i;
            }

            if t < v {
                bad = true;
            }
        }

        if !bad {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", r);
}
