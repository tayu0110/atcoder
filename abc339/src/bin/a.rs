fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    println!("{}", s.trim().split('.').next_back().unwrap());
}
