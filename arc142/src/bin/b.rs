use proconio::input;

fn main() {
    input! {n: usize};

    let mut front = 0;
    let mut back = n-1;

    for i in 0..n {
        if i % 2 == 0 {
            for j in 0..n {
                if j > 0 {
                    print!(" ");
                }
                print!("{}", front * n + j + 1);
            }
            println!();
            front += 1;
        } else {
            for j in 0..n {
                if j > 0 {
                    print!(" ");
                }
                print!("{}", back * n + j + 1);
            }
            println!();
            back -= 1;
        }
    }
}