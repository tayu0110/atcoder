use proconio::input;

fn main() {
    input! {mut a: usize, mut b: usize}

    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }

    println!("{}", a)
}
