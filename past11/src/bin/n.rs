use flow::Dinic;
use itertools::Itertools;
use proconio::*;

fn main() {
    input! {h: usize, w: usize, c: [[usize; w]; h]}

    // in: (i*w+j)*2, out: (i*w+j)*2+1
    let src = |i: usize, j: usize| (i * w + j) * 2;
    let dst = |i: usize, j: usize| src(i, j) + 1;
    let mut ff = Dinic::<usize>::new(h * w * 2);
    for i in 0..h {
        for j in 0..w {
            ff.set_edge(src(i, j), dst(i, j), c[i][j]);
            for (dx, dy) in [(0, 1), (1, 0), (0, !0), (!0, 0)] {
                let ni = i.wrapping_add(dx);
                let nj = j.wrapping_add(dy);
                if ni < h && nj < w {
                    ff.set_edge(dst(i, j), src(ni, nj), usize::MAX);
                }
            }
        }
    }

    println!("{}", ff.flow(dst(0, 0), src(h - 1, w - 1)));
    let min_cut = ff.min_cut_restoration();
    let mut res = vec![vec!['.'; w]; h];
    for (from, e) in min_cut {
        let to = e.to;
        if from / 2 == to / 2 {
            let id = from / 2;
            if id == 0 || id == h * w - 1 {
                continue;
            }
            let i = id / w;
            let j = id % w;
            res[i][j] = '#';
        }
    }

    for r in res {
        println!("{}", r.iter().join(""))
    }
}
