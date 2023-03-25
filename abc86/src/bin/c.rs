use proconio::*;

fn main() {
    input! {n: usize, mut p: [(i32, i32, i32); n]}
    p.insert(0, (0, 0, 0));

    for v in p.windows(2) {
        let (pt, px, py) = v[0];
        let (t, x, y) = v[1];

        let d = (px - x).abs() + (py - y).abs() - (t - pt);

        if d > 0 {
            println!("No");
            return;
        }

        if d.abs() % 2 != 0 {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
