use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {x0: usize, y0: usize, r0: usize, x1: usize, y1: usize, r1: usize}

        let dr = (r0 + r1) * (r0 + r1);
        let dx = x0.abs_diff(x1);
        let dy = y0.abs_diff(y1);
        let dxy = dx * dx + dy * dy;

        if dxy <= dr && dxy >= r0.abs_diff(r1) * r0.abs_diff(r1) {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
