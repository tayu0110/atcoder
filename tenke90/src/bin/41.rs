use proconio::input;

fn crs((x1, y1): (i64, i64), (x2, y2): (i64, i64), (x3, y3): (i64, i64)) -> i64 {
    let (x1, y1, x2, y2) = (x1 - x2, y1 - y2, x3 - x2, y3 - y2);
    x1 * y2 - x2 * y1
}

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    input! {n: usize, mut p: [(i64, i64); n]};
    p.sort();

    let mut g1 = vec![p[0], p[1]];
    let mut g2 = vec![p[0], p[1]];
    for i in 2..n {
        while g1.len() >= 2 && crs(g1[g1.len()-1], g1[g1.len()-2], p[i]) <= 0 {
            g1.pop();
        }
        while g2.len() >= 2 && crs(g2[g2.len()-1], g2[g2.len()-2], p[i]) >= 0 {
            g2.pop();
        }
        g1.push(p[i]);
        g2.push(p[i]);
    }

    let mut t = g1;
    t.append(&mut g2.into_iter().skip(1).rev().skip(1).collect());
    
    let mut ep = t.len() as i64;
    let mut s = 0;
    for i in 0..t.len() {
        let (ax, ay) = t[i];
        let (bx, by) = t[(i+1)%t.len()];
        let (vx, vy) = ((ax - bx).abs(), (ay - by).abs());
        let r = gcd(vx, vy);
        ep += r - 1;
        s += (bx - ax) * (by + ay);
    }
    s = s.abs();

    let res = (s + ep + 2) / 2 - n as i64;
    println!("{}", res);
}