use proconio::input;
	
fn main() {
    input! {s: [usize; 4]};

    let mut res = 0;
    for v in s {
        if v == 1111 {
            res += 1;
        }
    }

    println!("{}", res);
}