use proconio::*;

fn main() {
    input! {n: i32, q: usize, query: [(char, i32); q]}

    let (mut l, mut r) = (0, 1);
    let mut res = 0;
    for (h, t) in query {
        let t = t - 1;

        let mut min = usize::MAX;
        let (mut pl, mut pr) = (0, 0);
        if h == 'L' {
            for d in [1, -1] {
                let mut cnt = 0;
                let (mut nl, nr) = (l, r);
                while nl != t && nl != nr {
                    nl += d;
                    nl = (nl + n) % n;
                    cnt += 1;
                }

                if nl != nr {
                    if cnt < min {
                        min = cnt;
                        (pl, pr) = (nl, nr);
                    }
                }
            }
        } else {
            for d in [1, -1] {
                let mut cnt = 0;
                let (nl, mut nr) = (l, r);
                while nr != t && nl != nr {
                    nr += d;
                    nr = (nr + n) % n;
                    cnt += 1;
                }

                if nr != nl {
                    if cnt < min {
                        min = cnt;
                        (pl, pr) = (nl, nr);
                    }
                    min = min.min(cnt);
                }
            }
        }

        (l, r) = (pl, pr);
        res += min;
    }

    println!("{res}")
}
