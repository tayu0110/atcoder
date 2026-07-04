use ds::{DynamicSequence, MapMonoid};
use proconio::*;

struct T;
#[allow(unused)]
impl MapMonoid for T {
    type M = char;
    type Act = ();
    fn e() -> Self::M {
        '\0'
    }
    fn op(l: &Self::M, _r: &Self::M) -> Self::M {
        *l
    }
    fn id() -> Self::Act {}
    fn composite(l: &Self::Act, r: &Self::Act) -> Self::Act {}
    fn map(m: &Self::M, act: &Self::Act) -> Self::M {
        *m
    }
}

fn main() {
    input! {_: usize, m: usize, s: String, t: String, query: [(usize, usize); m]}

    let mut s = s.chars().collect::<DynamicSequence<T>>();
    let mut t = t.chars().collect::<DynamicSequence<T>>();

    for (l, r) in query {
        let s3 = s.split_off(r);
        let s2 = s.split_off(l - 1);
        let t3 = t.split_off(r);
        let t2 = t.split_off(l - 1);

        s.append(t2);
        s.append(s3);

        t.append(s2);
        t.append(t3);
    }

    println!("{}", s.into_iter().collect::<String>());
}
