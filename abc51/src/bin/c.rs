use proconio::*;

fn main() {
    input! {sx: i32, sy: i32, tx: i32, ty: i32}

    let v = vec![
        (sx, sy + 1, tx - 1, ty),
        (tx, ty + 1, sx - 1, sy),
        (sx + 1, sy, tx, ty - 1),
        (tx + 1, ty, sx, sy - 1),
    ];
    let w = vec![('U', 'R'), ('U', 'R'), ('R', 'U'), ('R', 'U')];

    let mut res = String::new();
    for (i, ((c, d), (mut sx, mut sy, tx, ty))) in w.into_iter().zip(v).enumerate() {
        res.push(c);
        while sx != tx || sy != ty {
            if i == 0 || i == 3 {
                if sy < ty {
                    sy += 1;
                    res.push('U');
                } else if sy > ty {
                    sy -= 1;
                    res.push('D');
                } else if sx > tx {
                    sx -= 1;
                    res.push('L');
                } else {
                    sx += 1;
                    res.push('R');
                }
            } else {
                if sx < tx {
                    sx += 1;
                    res.push('R');
                } else if sx > tx {
                    sx -= 1;
                    res.push('L');
                } else if sy > ty {
                    sy -= 1;
                    res.push('D');
                } else {
                    sy += 1;
                    res.push('U');
                }
            }
        }
        res.push(d);
    }

    println!("{}", res);
}
