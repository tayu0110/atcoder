use proconio::input;

fn main() {
    input! {a: [usize; 5]}
    let a = a.into_iter().collect::<std::collections::HashSet<_>>();
    println!("{}", a.len())
}
