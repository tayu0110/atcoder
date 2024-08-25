#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {s: Chars};

    let mut map: std::collections::BTreeMap<char, i32> = std::collections::BTreeMap::new();
    for v in s {
        if let std::collections::btree_map::Entry::Vacant(e) = map.entry(v) {
            e.insert(1);
        } else {
            *map.entry(v).or_insert(0) += 1;
        }
    }

    for (k, v) in map {
        if v == 1 {
            println!("{}", k);
            std::process::exit(0);
        }
    }

    println!("-1");
}
