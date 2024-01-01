use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    if n == 2 {
        if (a[0] + a[1]) % 2 == 0 {
            let h = (a[0] + a[1]) / 2;
            if a[0] >= h {
                println!("{}", a[0] - h);
            } else {
                println!("-1");
            }
        } else {
            println!("-1")
        }
        return;
    }

    let (a, b, c) = (a[0], a[1], a[2..].iter().fold(0, |s, v| s ^ v));
    let s = a + b;

    if s < c || (s - c) % 2 != 0 {
        println!("-1");
        return;
    }

    let d = (s - c) / 2;
    if d > a || d & c != 0 {
        println!("-1");
        return;
    }

    let mut res = 0;
    for i in (0..50).rev() {
        if c & (1 << i) != 0 && d ^ res ^ (1 << i) <= a {
            res |= 1 << i;
        }
    }

    if res == 0 {
        println!("-1");
    } else {
        println!("{}", a - (d ^ res));
    }
}
