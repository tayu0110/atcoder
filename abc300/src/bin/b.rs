use proconio::*;

fn main() {
    input! {h: usize, w: usize, mut a: [marker::Chars; h], b: [marker::Chars; h]}

    for _ in 0..h {
        for _ in 0..w {
            if a == b {
                println!("Yes");
                return;
            }
            a.iter_mut().for_each(|v| v.rotate_left(1));
        }

        a.rotate_left(1);
    }

    println!("No")
}
