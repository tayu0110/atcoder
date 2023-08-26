use proconio::input;

fn main() {
    input! {n: usize, mut p: [(usize, usize); n]}

    let mut res = vec![true; n];
    for i in 0..n {
        if !res[i] {
            continue;
        }
        let (a, b) = p[i];
        for j in 0..n {
            if i == j {
                continue;
            }

            let (na, nb) = p[j];

            let s = (na + b - 1) / b;
            let t = (a + nb - 1) / nb;

            if s < t {
                res[j] = false;
            } else if s > t {
                res[i] = false;
            } else {
                res[i] = false;
                res[j] = false;
            }
        }
    }

    if !res.iter().fold(false, |s, v| s | v) {
        println!("-1")
    } else {
        println!("{}", res.iter().position(|b| *b).unwrap() + 1);
    }
}
