fn main() {
    let f = |n: usize| (n * n + 4) / 8;
    println!("{}", f(f(f(20))))
}
