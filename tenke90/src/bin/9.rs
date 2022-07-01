use proconio::input;

fn main() {
    input! {n: usize, p: [(i64, i64); n]};

    let mut res = 0f64;
    for (i, (x, y)) in p.iter().enumerate() {
        let mut np = p.iter().enumerate().filter(|(j, _)| i != *j).map(|(_, (nx, ny))| (nx - *x, ny - *y)).collect::<Vec<(i64, i64)>>();
        np.sort_by(|(x0, y0), (x1, y1)| {
            ((*y0, *x0) < (0, 0)).cmp(&((*y1, *x1) < (0, 0))).then_with(|| (*x1 * *y0).cmp(&(*x0 * *y1)))
        });
        np.append(&mut np.clone());
        let mut l = 0;
        let mut r = 0;
        while l < np.len() {
            let (x, y) = np[l];
            r = std::cmp::max(l, r);
            while r < np.len() {
                let (nx, ny) = np[r];
                let (tx, ty) = (x * nx + y * ny, x * ny - y * nx);
                let k = (ty as f64).atan2(tx as f64).abs() * 180.0 / std::f64::consts::PI;
                if k > res && (nx != x || ny != y) {
                    res = k;
                }
                if ty < 0 {
                    break;
                }
                r += 1;
            }
            l += 1;
        }
    }

    println!("{}", res);
}