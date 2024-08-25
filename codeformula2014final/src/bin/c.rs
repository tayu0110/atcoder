fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let mut set = std::collections::BTreeSet::new();
    for s in s.split_ascii_whitespace() {
        if s.contains('@') {
            let s = s.split('@').collect::<Vec<_>>();
            for s in s.into_iter().skip(1) {
                if !s.is_empty() {
                    set.insert(s.to_string());
                }
            }
        }
    }

    for s in set {
        println!("{}", s);
    }
}
