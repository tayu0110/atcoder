use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {mut a: [i64; 3]}
        a.sort();

        let (a, b, c) = (a[0], a[1], a[2]);

        if a == b && b == c {
            println!("0");
            continue;
        }

        let (a, b, c) = (0, b - a, c - a);
        if (a + b + c) % 3 != 0 {
            println!("-1");
            continue;
        }

        let target = (a + b + c) / 3;
        let (a, b, c) = (a - target, b - target, c - target);

        if a.abs() % 2 != 0 || b.abs() % 2 != 0 || c.abs() % 2 != 0 {
            println!("-1");
            continue;
        }

        if b >= 0 {
            println!("{}", (b + c) / 2);
        } else {
            println!("{}", c / 2);
        }
    }
}
