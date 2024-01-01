use proconio::*;

fn main() {
    input! {n: usize, x: [i64; n], l: [i64; n]}

    let mut s = vec![];
    for &x in &x {
        for &l in &l {
            s.push(x + l);
            s.push(x - l - 1);
        }
    }

    s.sort();

    let mut res = 0;
    for v in s.windows(2) {
        let k = v[1];
        let mut y = vec![];
        for &x in &x {
            y.push((k - x).abs());
        }

        y.sort();
        if l.iter().zip(y).all(|(&l, y)| y <= l) {
            res += k - v[0];
        }
    }

    println!("{res}")
}
