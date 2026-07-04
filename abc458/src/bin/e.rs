use proconio::*;
use static_modint::{combination, Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {x: u32, y: u32, z: u32}

    let com = combination::<Mod998244353>(x + y + z + 10);
    let space = y + 1;
    let mut ret = Modint::zero();
    for fx in 1..space {
        if fx <= x {
            let fz = space - fx;
            ret += com(space, fx) * com(x - 1, x - fx) * com(z + fz - 1, z);
            // eprintln!(
            //     "fx: {fx}, ret: {ret}, com(space, x): {}, com(x+fx-1,x-fx): {}, com(z+fz-1,z): {}",
            //     com(space, x),
            //     com(x - 1, x - fx),
            //     com(z + fz - 1, z)
            // );
        }
    }
    println!("{ret}")
}
