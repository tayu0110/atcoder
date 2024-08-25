use polynomial::Polynomial;
use proconio::*;
use static_modint::Mod1000000007;

fn main() {
    input! {n: usize, k: usize, a: [u32; n]}

    let mut stack = a
        .into_iter()
        .map(|a| (0..=a).map(|_| 1u32).collect::<Polynomial<Mod1000000007>>())
        .collect::<Vec<_>>();

    while stack.len() > 1 {
        let mut new = Vec::with_capacity((stack.len() + 1) / 2);
        while let Some(mut p) = stack.pop() {
            if let Some(q) = stack.pop() {
                p = (p * q).prefix(k + 1);
            }
            new.push(p);
        }
        stack = new;
    }

    let res = stack.pop().unwrap().prefix(k + 1);
    println!("{}", res[k]);
}
