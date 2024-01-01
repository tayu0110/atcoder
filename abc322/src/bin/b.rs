use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: String, t: String}

    let f = t.starts_with(&s);
    let g = t.ends_with(&s);

    if f && g {
        println!("0")
    } else if f && !g {
        println!("1")
    } else if !f && g {
        println!("2")
    } else {
        println!("3")
    }
}
