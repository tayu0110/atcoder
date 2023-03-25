use proconio::input;

fn main() {
    input! {h: usize, w: usize, d: usize, a: [[usize; w]; h], q: usize, q: [(usize, usize); q]}

    let mut t = vec![(0, 0); h * w];
    for i in 0..h {
        for j in 0..w {
            t[a[i][j] - 1] = (i as i32, j as i32);
        }
    }

    let mut u = vec![vec![]; d];
    for i in 0..d {
        let v = &mut u[i];
        let mut prev = (0, 0);
        for j in (i..h * w).step_by(d) {
            if v.is_empty() {
                v.push(0);
            } else {
                let last = *v.last().unwrap();
                v.push(
                    last + (prev.0 - t[j % (h * w)].0).abs() + (prev.1 - t[j % (h * w)].1).abs(),
                );
            }

            prev = t[j % (h * w)];
        }
    }

    for (l, r) in q {
        let (l, r) = (l - 1, r - 1);

        let v = &u[l % d];

        println!("{}", v[r / d] - v[l / d]);
    }
}
