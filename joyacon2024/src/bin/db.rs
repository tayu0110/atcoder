use proconio::*;

fn main() {
    input! {s: String}

    let s = s.split('/').collect::<Vec<_>>();

    if s[0] < "2019" {
        println!("Heisei")
    } else if s[0] > "2019" {
        println!("TBD")
    } else if s[1] < "04" {
        println!("Heisei")
    } else if s[1] > "04" {
        println!("TBD")
    } else if s[2] <= "30" {
        println!("Heisei")
    } else {
        println!("TBD")
    }
}
