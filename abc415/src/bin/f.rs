use ds::{Monoid, SegmentTree};
use proconio::*;

struct T;
impl Monoid for T {
    // (kind, length)
    type M = Vec<(u8, usize)>;

    fn id() -> Self::M {
        vec![]
    }

    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        let mut new = if !l.is_empty() && !r.is_empty() && l.last().unwrap().0 == r[0].0 {
            [
                &l[..l.len() - 1],
                [(r[0].0, l.last().unwrap().1 + r[0].1)].as_slice(),
                &r[1..],
            ]
            .concat()
        } else {
            [l.as_slice(), r.as_slice()].concat()
        };
        if new.len() > 3 {
            let len = new.len();
            let max = new.iter().max_by_key(|l| l.1).copied().unwrap();
            let pos = new.iter().position(|&v| v == max).unwrap();
            for i in (1..len - 1).rev() {
                if i != pos {
                    new.remove(i);
                }
            }
        }
        new
    }
}

fn main() {
    input! {_: usize, q: usize, s: marker::Bytes}

    let mut st = s
        .into_iter()
        .map(|c| vec![(c, 1)])
        .collect::<SegmentTree<T>>();

    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {i: usize, x: char}
            st.set(i - 1, vec![(x as u8, 1)]);
        } else {
            input! {l: usize, r: usize}
            let res = st.fold(l - 1..r);
            let &(_, res) = res.iter().max_by_key(|res| res.1).unwrap();
            println!("{}", res);
        }
    }
}
