use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}

    for i in 0..=2000 {
        if !a.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}
