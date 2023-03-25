#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut a: [i64; n]}

    a.iter_mut().enumerate().for_each(|(i, v)| *v -= i as i64 + 1);

    let (mut l, mut r) = (*a.iter().min().unwrap()-1, *a.iter().max().unwrap()+1);

    while r - l > 2 {
        let (p, q) = ((l*2+r) / 3, (l+r*2) / 3);
        let ps = a.iter().fold(0, |s, v| s + (*v-p).abs());
        let qs = a.iter().fold(0, |s, v| s + (*v-q).abs());

        if ps < qs {
            r = q;
        } else {
            l = p;
        }
    }

    let mut res = std::i64::MAX;
    for b in l-5..=r+5 {
        let t = a.iter().fold(0, |s, v| s + (*v - b).abs());
        res = std::cmp::min(res, t);
    }

    println!("{}", res);
}
