use proconio::*;
use string::RollingHash;

fn main() {
    input! {s: String}

    let mut res = 0;
    let hash = RollingHash::new(&s);
    for i in 1..s.len() {
        let front = i;
        let back = s.len() - front;
        if front < back + 1 {
            continue;
        }

        let f = {
            let (mut l, mut r) = (0, back - 1);
            while r - l > 1 {
                let m = (r + l) / 2;
                if hash.get(..m) != hash.get(front..front + m) {
                    r = m;
                } else {
                    l = m;
                }
            }

            if hash.get(..r) == hash.get(front..front + r) {
                r
            } else {
                r - 1
            }
        };
        let b = {
            let (mut l, mut r) = (0, back - 1);
            while r - l > 1 {
                let m = (r + l) / 2;
                if hash.get(front - m..front) != hash.get(s.len() - m..) {
                    r = m;
                } else {
                    l = m;
                }
            }
            if hash.get(front - r..front) == hash.get(s.len() - r..) {
                r
            } else {
                r - 1
            }
        };

        if f == 0 || b == 0 {
            continue;
        }

        res += (f + b).saturating_sub(back - 1);
    }

    println!("{res}");
}
