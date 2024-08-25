use proconio::*;

fn main() {
    input! {a: [usize; 4]}

    let sum = a.iter().sum::<usize>();
    println!("{}", sum / 60);
    println!("{}", sum % 60);
}
