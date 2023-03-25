use proconio::input;

fn main() {
    input! {k: usize}

    if k < 60 {
        println!("21:{:02}", k);
    } else {
        println!("22:{:02}", k - 60);
    }
}
