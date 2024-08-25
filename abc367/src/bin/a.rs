use proconio::*;

fn main() {
    input! {a: usize, mut b: usize, c: usize}

    if c > b {
        b += 24;
    }

    if (c..=b).contains(&a) || (c..=b).contains(&(a + 24)) {
        println!("Yes")
    } else {
        println!("No")
    }
}
