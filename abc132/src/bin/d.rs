use modint::{combination, Mod1000000007, StaticModint};
use proconio::input;

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize, k: usize}

    let com = combination(100000);
    println!("{}", n - k + 1);
    for i in 2..=k {
        if k - 1 < i - 1 {
            println!("0");
            continue;
        }
        let c = com(k - 1, i - 1);
        let mut nc = Modint::zero();
        for d in vec![i - 1, i, i + 1] {
            if n - k > 0 {
                nc += com(n - k - 1, d - 1);
                if d == i {
                    nc += com(n - k - 1, d - 1);
                }
            }
        }

        println!("{}", c * nc);
    }
}
