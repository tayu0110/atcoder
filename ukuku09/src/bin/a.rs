use proconio::*;

fn main() {
    input! {t: usize, x: [usize; t]}
    for x in x {
        let mut res = std::usize::MAX;
        let mut a = (0, 1);
        let mut b = (1, 0);
        while a.0 + b.1 <= x {
            if (x - b.1) % b.0 == 0 {
                res = (x - b.1) / b.0;
            }
            a.0 += b.0;
            a.1 += b.1;
            std::mem::swap(&mut a, &mut b);
        }
        println!("1 {}", res);
    }
}
