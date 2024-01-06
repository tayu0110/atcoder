use proconio::input;
use segtree::{Monoid, SegmentTree};

struct I64Add;

impl Monoid for I64Add {
    type M = i64;
    fn id() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        l + r
    }
}

fn main() {
    input! {n: usize, p: [(i64, i64); n], q: usize}

    let mut query = vec![];
    for _ in 0..q {
        input! {t: usize, x: usize}

        if t == 1 || t == 2 {
            input! {y: i64}
            query.push((t, x, y));
        } else {
            query.push((t, x, 0));
        }
    }

    let (mut a, mut b) = p.into_iter().unzip::<i64, i64, Vec<_>, Vec<_>>();
    let (map, reverse_map) = {
        let mut map = std::collections::BTreeMap::new();
        for &a in &a {
            map.insert(a, 0);
        }
        for &(t, _, y) in &query {
            if t != 3 {
                map.insert(y, 0);
            }
        }
        let mut cnt = 0usize;
        let mut rev = vec![];
        for (k, v) in map.iter_mut() {
            *v = cnt;
            cnt += 1;
            rev.push(*k);
        }
        (map, rev)
    };

    let mut st = SegmentTree::<I64Add>::new(map.len());
    let mut num = SegmentTree::<I64Add>::new(map.len());
    for (&a, &b) in a.iter().zip(b.iter()) {
        let index = *map.get(&a).unwrap();
        st.update_by(index, |&old| old + a * b);
        num.update_by(index, |&l| l + b);
    }

    for (t, x, y) in query {
        if t == 1 {
            let x = x - 1;
            let old = a[x];
            a[x] = y;

            if old == y {
                continue;
            }

            let oi = *map.get(&old).unwrap();
            let yi = *map.get(&y).unwrap();

            let val = old * b[x];
            st.update_by(oi, |&old| {
                assert!(old >= val);
                old - val
            });
            num.update_by(oi, |&l| {
                assert!(l >= b[x]);
                l - b[x]
            });
            st.update_by(yi, |&old| old + y * b[x]);
            num.update_by(yi, |&l| l + b[x]);
        } else if t == 2 {
            let x = x - 1;
            let old = b[x];
            b[x] = y;

            if old == y {
                continue;
            }

            let index = *map.get(&a[x]).unwrap();

            let val = a[x] * (y - old);
            st.update_by(index, |&old| {
                assert!(old + val >= 0);
                old + val
            });
            num.update_by(index, |&l| {
                assert!(l + y - old >= 0);
                l + y - old
            });
        } else {
            let (mut l, mut r) = (-1, map.len() as i32);
            while r - l > 1 {
                let m = (r + l) / 2;
                let k = num.fold(m as usize..map.len());
                if k <= x as i64 {
                    r = m;
                } else {
                    l = m;
                }
            }

            let k = num.fold(r as usize..map.len());
            if k == x as i64 {
                println!("{}", st.fold(r as usize..map.len()))
            } else if k < x as i64 {
                if r == 0 {
                    println!("-1")
                } else {
                    assert!(num.fold(l as usize..l as usize + 1) >= 0);
                    println!(
                        "{}",
                        st.fold(r as usize..map.len()) + reverse_map[l as usize] * (x as i64 - k)
                    )
                }
            } else {
                unreachable!();
            }
        }
    }
}
