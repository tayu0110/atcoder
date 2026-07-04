use flow::Dinic;
use proconio::*;

fn main() {
    input! {n: usize, s: [(f64, f64); n], g: [(f64, f64); n]}

    let (mut l, mut r) = (0.0, 2e18);
    for _ in 0..150 {
        let m = (r + l) / 2.0;
        let mut ff = Dinic::new(n * 2 + 2);
        for (i, (sx, sy)) in s.iter().enumerate() {
            ff.set_edge(n * 2, i, 1);
            ff.set_edge(i + n, n * 2 + 1, 1);
            for (j, (gx, gy)) in g.iter().enumerate() {
                let d = (sx - gx).hypot(sy - gy);
                if d <= m {
                    ff.set_edge(i, j + n, 1);
                }
            }
        }

        let flow = ff.flow(n * 2, n * 2 + 1);
        if flow == n {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{r:.15}");
}
