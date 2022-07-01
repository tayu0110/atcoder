use proconio::input;

fn search(v: &Vec<i64>) -> i64 {
    let mut l = *v.first().unwrap();
    let mut r = *v.last().unwrap();
    while r - l > 2 {
        let nl = (2*l + r) / 3;
        let nr = (l + 2*r) / 3;
        let mut dl = 0;
        let mut dr = 0;
        for w in v {
            dl += (*w - nl).abs();
            dr += (*w - nr).abs();
            // dl += w.abs_diff(nl);
            // dr += w.abs_diff(nr);
        }

        // eprintln!("nl: {}, nr: {}, dl: {}, dr: {}", nl, nr, dl, dr);

        if dl < dr {
            r = nr;
        } else {
            l = nl;
        }
    }

    let mut res = 111122233344455566;
    for t in l..r+1 {
        let mut d = 0;
        for w in v {
            // d += w.abs_diff(t);
            d += (*w - t).abs();
        }
        // eprintln!("t: {}, d: {}", t, d);
        res = std::cmp::min(res, d);
    }
    res as i64
}

fn main() {
    input! {n: usize, p: [(i64, i64); n]};

    let mut x = vec![];
    let mut y = vec![];

    for (nx, ny) in p {
        x.push(nx);
        y.push(ny);
    }

    x.sort();
    y.sort();

    println!("{}", search(&x) + search(&y));
}