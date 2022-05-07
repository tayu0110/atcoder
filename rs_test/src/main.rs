use std::collections::BTreeSet;

fn main() {
    let mut se = BTreeSet::new();
    se.insert(2);
    se.insert(2);
    se.insert(4);
    se.insert(10);

    let res = *se.iter().next().unwrap();
    println!("{}", res);
    se.remove(&res);
    for v in se {
        println!("rem: {}", v);
    }
}
