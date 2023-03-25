#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {q: usize}

    const ROOT: i64 = 1000_000_010;

    let mut par_val_list = std::collections::HashMap::new();
    let mut now = ROOT;
    let mut used = ROOT;
    // (par, val)
    par_val_list.insert(now, (ROOT, -1));

    let mut res = vec![];

    for _ in 0..q {
        input! {query: String}

        match query.as_str() {
            "ADD" => {
                input! {x: i64}
                par_val_list.insert(used+1, (now, x));
                used += 1;
                now = used;
            }
            "DELETE" => {
                let (par, _) = *par_val_list.get(&now).unwrap();
                now = par;
            }
            "SAVE" => {
                input! {y: i64}
                *par_val_list.entry(y).or_insert((0, 0)) = (now, 0);
            }
            "LOAD" => {
                input! {z: i64}
                if let Some((saved, _)) = par_val_list.get(&z) {
                    now = *saved;
                } else {
                    now = ROOT;
                }
            }
            _ => {
                unreachable!();
            }
        }

        let (_, val) = *par_val_list.get(&now).unwrap();
        res.push(val);
    }

    println!("{}", res.iter().join(" "));
}
