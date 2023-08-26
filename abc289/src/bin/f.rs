use proconio::input;

fn main() {
    input! {mut sx: i64, mut sy: i64, tx: i64, ty: i64, a: i64, b: i64, c: i64, d: i64}

    if sx % 2 != tx % 2 || sy % 2 != ty % 2 {
        println!("No");
        return;
    }

    let pos0 = (a != b || sx == tx) && (c != d || sy == ty);
    let pos1 = (a != b || sx + tx == a + b) && (c != d || sy + ty == c + d);

    if !pos0 && !pos1 {
        println!("No");
        return;
    }

    println!("Yes");

    let inst = |x: i64, y: i64, sx: &mut i64, sy: &mut i64| {
        println!("{} {}", x, y);
        *sx = 2 * x - *sx;
        *sy = 2 * y - *sy;
    };

    if !pos0 {
        inst(a, c, &mut sx, &mut sy);
    }

    while sx < tx {
        inst(a, c, &mut sx, &mut sy);
        inst(a + 1, c, &mut sx, &mut sy);
    }
    while sx > tx {
        inst(a + 1, c, &mut sx, &mut sy);
        inst(a, c, &mut sx, &mut sy);
    }
    while sy < ty {
        inst(a, c, &mut sx, &mut sy);
        inst(a, c + 1, &mut sx, &mut sy);
    }
    while sy > ty {
        inst(a, c + 1, &mut sx, &mut sy);
        inst(a, c, &mut sx, &mut sy);
    }
}
