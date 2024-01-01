use proconio::*;

fn main() {
    input! {s: String}

    let res = match &s[..2] {
        "Su" | "Sa" => 0,
        "Fr" => 1,
        "Th" => 2,
        "We" => 3,
        "Tu" => 4,
        "Mo" => 5,
        _ => {
            unreachable!()
        }
    };

    println!("{res}")
}
