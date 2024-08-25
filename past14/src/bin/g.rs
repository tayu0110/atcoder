use proconio::*;

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h]}

    let mut res = vec![];
    for i in 0..h {
        for j in 0..w {
            let k = a[i][j];
            for (dy, dx) in [(0, 1),
                (1, 0),
                (0, 1usize.wrapping_neg()),
                (1usize.wrapping_neg(), 0)] {
                let ni = i.wrapping_add(dy);
                let nj = j.wrapping_add(dx);

                if ni < h && nj < w {
                    let l = a[ni][nj];
                    res.push((k.min(l), k.max(l)))
                }
            }
        }
    }

    res.sort();
    res.dedup();

    for (x, y) in res {
        println!("{} {}", x, y)
    }
}
