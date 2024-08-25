use proconio::input;

fn main() {
    input! {k: i64, x: i64, y: i64}

    let (tx, ty) = (x - y, x + y);
    if tx.abs() % 2 != ty.abs() % 2 {
        println!("-1");
        return;
    }
    if k % 2 != tx.abs() % 2 {
        println!("-1");
        return;
    }

    let (mut nx, mut ny) = (0, 0);
    let mut res = vec![];
    while nx != tx || ny != ty {
        let (kx, ky) = if nx < tx && ny < ty {
            (k, k)
        } else if nx > tx && ny < ty {
            (-k, k)
        } else if nx > tx && ny > ty {
            (-k, -k)
        } else {
            (k, -k)
        };
        if nx == tx {
            let nk = if ny < ty { k } else { -k };
            if ny + nk == ty {
                ny += nk;
                if (nx + ny) % 2 == 1 {
                    nx += kx;
                }
            } else if (ty - ny).abs() >= 2 * k {
                ny += nk;
                if (nx + ny) % 2 == 1 {
                    nx += kx;
                }
            } else {
                nx += nk;
                ny = ty - nk;
            }
        } else if ny == ty {
            let nk = if nx < tx { k } else { -k };
            if nx + nk == tx {
                nx += nk;
                if (nx + ny) % 2 == 1 {
                    ny += ky;
                }
            } else if (tx - nx).abs() >= 2 * k {
                nx += nk;
                if (nx + ny) % 2 == 1 {
                    ny += ky;
                }
            } else {
                ny += nk;
                nx = tx - nk;
            }
        } else if (tx - nx).abs() < k && (ty - ny).abs() < k {
            nx -= kx;
            ny = ty;
            res.push((nx, ny));
            ny += ky;
            nx = tx - kx;
            res.push((nx, ny));
            nx += kx;
            ny -= ky;
        } else if (tx - nx).abs() == k && (ty - ny).abs() == k {
            nx += kx;
            ny += ky;
        } else if (tx - nx).abs() == k && (ty - ny).abs() < k {
            nx += kx;
            ny = ty - ky;
        } else if (tx - nx).abs() < k && (ty - ny).abs() == k {
            ny += ky;
            nx = tx - kx;
        } else if (tx - nx).abs() <= 2 * k {
            nx = tx - kx;
            ny += ky;
        } else if (ty - ny).abs() <= 2 * k {
            ny = ty - ky;
            nx += kx;
        } else {
            nx += kx;
            ny += ky;
        }
        res.push((nx, ny));
    }

    println!("{}", res.len());
    for (x, y) in res {
        println!("{} {}", (x + y) / 2, (y - x) / 2);
    }
}
