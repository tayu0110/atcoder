use proconio::*;

const DX: [usize; 4] = [!0, 0, 1, 0];
const DY: [usize; 4] = [0, !0, 0, 1];

fn main() {
    input! {n: usize}

    let mut a = vec![vec![false; 1000]; 1000];
    let (mut x, mut y) = (500, 500);
    let mut k = 0;
    let mut b = vec![];
    for _ in 0..30000 {
        b.push(a[y][x]);
        if a[y][x] {
            k = (k + 1) % 4;
        } else {
            k = (k + 3) % 4;
        }
        a[y][x] = !a[y][x];
        x = x.wrapping_add(DX[k]);
        y = y.wrapping_add(DY[k]);
    }

    if n < 30000 {
        println!("{}", b[n] as usize);
    } else {
        println!("{}", b[30000 + (n - 30000) % 104 - 104] as usize);
    }
}
