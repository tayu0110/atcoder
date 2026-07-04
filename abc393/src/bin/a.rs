use proconio::*;

fn main() {
    input! {s1: String, s2: String}

    let is_sick = |s: &str| s == "sick";
    println!(
        "{}",
        match (is_sick(&s1), is_sick(&s2)) {
            (true, true) => 1,
            (true, false) => 2,
            (false, true) => 3,
            (false, false) => 4,
        }
    )
}
