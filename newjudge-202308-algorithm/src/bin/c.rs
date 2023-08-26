use std::{collections::BTreeSet, io::Write};

use proconio::*;

fn main() {
    input_interactive!(n: usize);

    let mut set = (1..=2 * n + 1).collect::<BTreeSet<_>>();
    for _ in 0..=n {
        let &first = set.first().unwrap();

        println!("{first}");
        std::io::stdout().flush().unwrap();

        input_interactive!(t: usize);

        set.remove(&first);
        set.remove(&t);
    }
}
