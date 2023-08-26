use proconio::input;

fn main() {
    input! {n: usize}

    for _ in 0..n {
        input! {mut a: u32, mut b: u32}

        while b > 0 {
            a %= b;
            std::mem::swap(&mut a, &mut b);
        }
        println!("{}", a)
    }
}
