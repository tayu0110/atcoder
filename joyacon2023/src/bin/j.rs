use proconio::input;

fn main() {
    input! {s:String}

    if s.starts_with('M') {
        println!("5");
    } else if s.starts_with('W') {
        println!("3");
    } else if s.starts_with('F') {
        println!("1");
    } else if s.starts_with("Th") {
        println!("2");
    } else {
        println!("4");
    }
}
