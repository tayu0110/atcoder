use proconio::*;

fn main() {
    input! {n: usize, e: [(i32, i32); n]}

    for &(x, y) in &e {
        let mut d = 0;
        let mut res = 0;
        for (i, &(nx, ny)) in e.iter().enumerate() {
            let dx = nx - x;
            let dy = ny - y;
            if d < dx * dx + dy * dy {
                d = dx * dx + dy * dy;
                res = i + 1;
            }
        }

        println!("{res}")
    }
}
