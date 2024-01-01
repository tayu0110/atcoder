use proconio::*;

fn main() {
    input! {n: usize}

    for n in n.. {
        let o = n % 10;
        let p = n % 100 / 10;
        let q = n / 100;

        if p * q == o {
            println!("{n}");
            return;
        }
    }
}
