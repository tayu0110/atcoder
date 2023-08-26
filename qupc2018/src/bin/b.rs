use proconio::*;

fn main() {
    input! {q: usize}

    for _ in 0..q {
        input! {a: usize, b: usize, c: usize}

        let sum = a * 100 + b * 10 + c;
        if sum % 2 == 1 {
            println!("No");
            continue;
        }

        let mut target = sum / 2;
        target -= 100 * (target / 100).min(a);
        target -= 10 * (target / 10).min(b);
        if target <= c {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
