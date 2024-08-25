use proconio::*;
use string::SuffixArray;

fn main() {
    input! {s: String, q: usize}

    let sa = SuffixArray::new(&s);

    for _ in 0..q {
        input! {t: String}

        let start = {
            let (mut l, mut r) = (!0, s.len());
            while r.wrapping_sub(l) > 1 {
                let m = r.wrapping_add(l) / 2;
                let index = sa.get(m);
                if &s[index..s.len().min(index + t.len())] < &t {
                    l = m;
                } else {
                    r = m;
                }
            }
            r
        };

        let end = {
            let (mut l, mut r) = (!0, s.len());
            while r.wrapping_sub(l) > 1 {
                let m = r.wrapping_add(l) / 2;
                let index = sa.get(m);
                if &s[index..s.len().min(index + t.len())] <= &t {
                    l = m;
                } else {
                    r = m;
                }
            }
            r
        };

        println!("{}", end - start);
    }
}
