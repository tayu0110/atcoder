use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: u32, m: u32}

    let (n, m) = (n.max(m), n.min(m));

    if n - m > 1 {
        println!("0");
        return;
    }

    let mut nn = Modint::one();
    for i in 1..=n {
        nn *= Modint::raw(i);
    }
    let mut mm = Modint::one();
    for i in 1..=m {
        mm *= Modint::raw(i);
    }

    if n == m {
        println!("{}", nn * mm * Modint::raw(2));
    } else {
        println!("{}", nn * mm);
    }
}
