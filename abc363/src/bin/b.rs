use proconio::*;

fn main() {
    input! {n: usize, t: usize, p: usize, mut l: [usize; n]}

    for i in 0.. {
        if l.iter().filter(|&&l| l >= t).count() >= p {
            println!("{i}");
            return;
        }

        l.iter_mut().for_each(|l| *l += 1);
    }
}
