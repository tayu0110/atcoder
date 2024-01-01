use flow::Dinic;
use proconio::*;

fn main() {
    input! {n: usize, m: u16}

    let mut ff = Dinic::new(n);
    for _ in 0..m {
        input! {a: u16, b: u16, c: u16}
        ff.set_edge(a as usize - 1, b as usize - 1, c);
    }

    println!("{}", ff.flow(0, n - 1));
}
