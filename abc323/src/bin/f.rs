use proconio::*;

fn main() {
    input! {xa: i64, ya: i64, mut xb: i64, mut yb: i64, mut xc: i64, mut yc: i64}

    xb -= xa;
    xc -= xa;

    yb -= ya;
    yc -= ya;

    if xb < 0 {
        xb = -xb;
        xc = -xc;
    }
    if yb < 0 {
        yb = -yb;
        yc = -yc;
    }

    let mut start = vec![];
    // up
    if !(xb == xc && yb + 1 == yc) {
        let cnt = if xb > 0 { yb + 1 + xb } else { yb + 1 + 2 };
        start.push((xb, yb + 1, cnt));
    }
    // down
    if !(xb == xc && yb - 1 == yc) {
        let cnt = if xb > 0 {
            if yb == 0 {
                xb + 1
            } else {
                xb + yb - 1
            }
        } else {
            yb - 1
        };
        start.push((xb, yb - 1, cnt));
    }
    // left
    if !(xb - 1 == xc && yb == yc) {
        let cnt = if xb > 0 { yb + xb - 1 } else { 1 + yb };
        start.push((xb - 1, yb, cnt));
    }
    // right
    if !(xb + 1 == xc && yb == yc) {
        let cnt = if xb > 0 {
            if yb == 0 {
                1 + xb + 1 + 1
            } else {
                xb + 1 + yb
            }
        } else {
            1 + yb
        };
        start.push((xb + 1, yb, cnt));
    }

    let mut res = i64::MAX;
    for (x, y, mut cnt) in start {
        let (mut xc, mut yc, mut x, mut y) = (xc - xb, yc - yb, x - xb, y - yb);
        let (xb, yb) = (0, 0);
        if xc < 0 {
            xc = -xc;
            x = -x;
        }
        if yc < 0 {
            yc = -yc;
            y = -y;
        }

        if x > 0 || y > 0 {
            cnt += 2;
            if x == xb {
                (x, y) = (xb - 1, yb);
            } else if y == yb {
                (x, y) = (xb, yb - 1);
            } else {
                unreachable!()
            }
        }
        if xb == xc {
            if x == xb {
                res = res.min(cnt + yc - yb);
            } else {
                res = res.min(cnt + yc - yb + 2);
            }
        } else if yb == yc {
            if y == yb {
                res = res.min(cnt + xc - xb);
            } else {
                res = res.min(cnt + xc - xb + 2);
            }
        } else {
            res = res.min(cnt + xc - xb + yc - yb + 2);
        }
    }

    println!("{res}");
}
