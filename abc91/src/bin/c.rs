use flow::Dinic;
use proconio::input;

fn main() {
    input! {n: usize, p: [(usize, usize); n], q: [(usize, usize); n]}

    // a,b: i, c,d: i+n, src: 2*n, dst: 2*n+1
    let mut ff = Dinic::new(2 * n + 2);

    for (i, (a, b)) in p.into_iter().enumerate() {
        ff.set_edge(2 * n, i, 1);
        for (j, &(c, d)) in q.iter().enumerate() {
            if a < c && b < d {
                ff.set_edge(i, j + n, 1);
            }
        }
    }

    for j in n..2 * n {
        ff.set_edge(j, 2 * n + 1, 1);
    }

    println!("{}", ff.flow(2 * n, 2 * n + 1));
}
