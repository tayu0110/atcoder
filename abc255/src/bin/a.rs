use proconio::input;

fn main() {
    input! {r: usize, c: usize, a: [[usize; 2]; 2]};

    let r = r-1;
    let c = c-1;
    println!("{}", a[r][c]);
}