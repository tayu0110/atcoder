use ds::{DynamicSortedSequence, MapMonoid};
use proconio::*;

const M: usize = 1000_000_000;

struct T;
impl MapMonoid for T {
    type M = usize;
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
    input! {q: usize}

    let mut ev = DynamicSortedSequence::<T>::new();
    let mut od = DynamicSortedSequence::<T>::new();
    let mut z = usize::MAX;
    for _ in 0..q {
        input! {y: usize}

        let x = if z == usize::MAX {
            y % M + 1
        } else {
            (y + z) % M + 1
        };

        ev.insert(x);
        let evi = ev.first_index_of(&x).unwrap();
        ev.remove_once(&x);
        od.insert(x);
        let odi = od.first_index_of(&x).unwrap();
        od.remove_once(&x);

        let evb = ev.split_off(evi);
        let odb = od.split_off(odi);
        ev.append(odb);
        od.append(evb);

        if (evi + odi) % 2 == 0 {
            ev.insert(x);
        } else {
            od.insert(x);
        }

        let res = ev.fold(..);
        println!("{}", res);
        z = res;
    }
}
