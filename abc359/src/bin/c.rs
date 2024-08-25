use proconio::*;

fn main() {
    input! {mut sx: i64, sy: i64, mut tx: i64, ty: i64}

    if sx > tx {
        sx = -(sx + 1);
        tx = -(tx + 1);
    }

    if sx < 0 {
        let d = sx;
        sx += d * 2;
        tx += d * 2;
    }

    if (sx + sy) % 2 == 0 {
        sx += 1;
    }

    let res = (sy - ty).abs();
    sx += res;

    if sx >= tx {
        println!("{res}")
    } else {
        println!("{}", res + ((tx - sx + 1) / 2).max(0))
    }
}
