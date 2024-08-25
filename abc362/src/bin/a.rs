use proconio::*;

fn main() {
    input! {r: usize, g: usize, b: usize, c: String}

    let res;
    if c == "Red" {
        res = g.min(b);
    } else if c == "Green" {
        res = r.min(b);
    } else {
        res = r.min(g);
    }

    println!("{res}")
}
