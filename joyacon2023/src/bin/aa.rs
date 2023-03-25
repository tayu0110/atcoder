use proconio::input;

fn main() {
    input! {mut x: usize, k: usize}

    for i in 0..k {
        for _ in 0..i {
            x /= 10;
        }

        if x % 10 < 5 {
            x = x / 10 * 10;
        } else {
            x = (x / 10 + 1) * 10;
        }

        for _ in 0..i {
            x *= 10;
        }
    }

    println!("{}", x);
}
