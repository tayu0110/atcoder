use math::factorize;
use proconio::input;

fn main() {
    input! {t: usize}

    let mut res = vec![];
    for _ in 0..t {
        input! {n: u64}
        let mut v = factorize(n);
        v.sort();
        if v[1] == v[0] {
            res.push((v[0], v[2]));
        } else {
            res.push((v[2], v[0]));
        }
    }

    assert_eq!(res.len(), t);

    for (p, q) in res {
        println!("{} {}", p, q);
    }
}
