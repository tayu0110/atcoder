use ds::LinkCutTree;
use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {n: usize, q: usize, query: [(usize, usize, usize); q]}

    let mut x = 0;
    let mut lct = <LinkCutTree>::new(n + 1);
    for (a, b, c) in query {
        let ty = 1 + (((a * (1 + x)) % M) % 2);
        let u = 1 + (((b * (1 + x)) % M) % n);
        let v = 1 + (((c * (1 + x)) % M) % n);

        // eprintln!("ty: {ty}, u: {u}, v: {v}");

        if ty == 1 {
            lct.link_flat(u, v).unwrap();
        } else {
            lct.evert(u);
            if let Some(par) = lct.parent(v) {
                lct.evert(par);
                if lct.parent(u) == Some(par) {
                    println!("{par}");
                    x = par;
                } else {
                    println!("0");
                    x = 0;
                }
            } else {
                println!("0");
                x = 0;
            }
        }
    }
}
