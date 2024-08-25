use proconio::*;

fn big_to_bit(sx: usize, sy: usize, tx: usize, ty: usize) -> usize {
    let max = sx.abs_diff(tx).max(sy.abs_diff(ty));
    max * 2
}

fn main() {
    input! {k: usize, sx: usize, sy: usize, tx: usize, ty: usize}

    if k == 1 {
        println!("{}", sx.abs_diff(tx) + sy.abs_diff(ty));
        return;
    }

    let bs = (sx / k + sy / k) % 2 == 1;
    let bt = (tx / k + ty / k) % 2 == 1;

    if bs && bt {
        println!("{}", big_to_bit(sx / k, sy / k, tx / k, ty / k));
    } else if bs {
        let x = tx / k;
        let y = ty / k;
        let mut res = usize::MAX;
        res = res.min(big_to_bit(sx / k, sy / k, x, y) + tx - x * k + ty - y * k);
        res = res.min(big_to_bit(sx / k, sy / k, x + 1, y) + (x + 1) * k - tx + ty - y * k);
        res = res.min(big_to_bit(sx / k, sy / k, x, y + 1) + tx - x * k + (y + 1) * k - ty);
        res =
            res.min(big_to_bit(sx / k, sy / k, x + 1, y + 1) + (x + 1) * k - tx + (y + 1) * k - ty);

        println!("{res}");
    } else if bt {
        let x = sx / k;
        let y = sy / k;
        let mut res = usize::MAX;
        res = res.min(big_to_bit(x, y, tx / k, ty / k) + sx - x * k + sy - y * k);
        res = res.min(big_to_bit(x + 1, y, tx / k, ty / k) + (x + 1) * k - tx + sy - y * k);
        res = res.min(big_to_bit(x, y + 1, tx / k, ty / k) + sx - x * k + (y + 1) * k - ty);
        res =
            res.min(big_to_bit(x + 1, y + 1, tx / k, ty / k) + (x + 1) * k - tx + (y + 1) * k - ty);

        println!("{res}");
    } else {
    }
}
