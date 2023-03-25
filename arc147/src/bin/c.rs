#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [(usize, usize); n]};

    let (mut l, mut r) = (0, 0x3f3f3f3f);

    while r - l > 10 {
        let (s, t) = ((l*2+r) / 3, (l+r*2) / 3);

        let (mut vs, mut vt) = (vec![], vec![]);
        for (l, r) in &p {
            let (l, r) = (*l, *r);
            if s < l {
                vs.push(l);
            } else if l <= s && s <= r {
                vs.push(s);
            } else {
                vs.push(r);
            }
            if t < l {
                vt.push(l);
            } else if l <= t && t <= r {
                vt.push(t);
            } else {
                vt.push(r);
            }
        }
        vs.sort();
        vt.sort();

        let mut sum_s = vs.iter().sum::<usize>();
        let mut sum_t = vt.iter().sum::<usize>();

        let (mut res_s, mut res_t) = (0, 0);
        for i in 0..n {
            sum_s -= vs[i];
            sum_t -= vt[i];
            res_s += sum_s - vs[i] * (n-1 - i);
            res_t += sum_t - vt[i] * (n-1 - i);
        }

        if res_s > res_t {
            l = s;
        } else {
            r = t;
        }
    }

    let mut res = 0x3f3f3f3f3f3f3f3f;
    for s in l..=r {
        let mut vs = vec![];
        for (l, r) in &p {
            let (l, r) = (*l, *r);
            if s < l {
                vs.push(l);
            } else if l <= s && s <= r {
                vs.push(s);
            } else {
                vs.push(r);
            }
        }
        vs.sort();

        let mut sum_s = vs.iter().sum::<usize>();

        let mut res_s = 0;
        for i in 0..n {
            sum_s -= vs[i];
            res_s += sum_s - vs[i] * (n-1 - i);
        }

        res = std::cmp::min(res, res_s);
    }

    println!("{}", res);
}
