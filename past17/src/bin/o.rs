use ds::{DynamicSortedSequence, MapMonoid};
use proconio::*;

struct T;

impl MapMonoid for T {
    type M = i64;
    type Act = ();

    fn e() -> Self::M {
        0
    }
    fn op(l: &Self::M, r: &Self::M) -> Self::M {
        l + r
    }

    fn id() -> Self::Act {}
    fn composite(_: &Self::Act, _: &Self::Act) -> Self::Act {}
    fn map(m: &Self::M, _: &Self::Act) -> Self::M {
        *m
    }
}

fn main() {
    input! {n: usize, mut a: [i64; n], q: usize}

    let mut seq = a.iter().cloned().collect::<DynamicSortedSequence<T>>();
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {k: usize, d: i64}
            seq.remove_once(&a[k - 1]);
            a[k - 1] += d;
            seq.insert(a[k - 1]);
        } else {
            input! {x: i64}

            if let Some(&lb) = seq.range(x..).next() {
                let index = seq.first_index_of(&lb).unwrap();
                println!(
                    "{}",
                    seq.fold(index..) - x * (n - index) as i64 + x * index as i64
                        - seq.fold(..index)
                );
            } else {
                println!("{}", x * n as i64 - seq.fold(..));
            }
        }
    }
}
