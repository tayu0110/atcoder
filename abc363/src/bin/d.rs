use proconio::*;

fn main() {
    input! {mut n: usize}

    if n <= 10 {
        println!("{}", n - 1);
        return;
    }

    n -= 10;
    for k in 2.. {
        let amount = if k % 2 == 0 {
            10usize.pow(k / 2) * 9 / 10
        } else {
            10usize.pow(k / 2) * 9
        };

        if n < amount {
            // https://www.geeksforgeeks.org/nth-palindrome-k-digits/
            let temp = if k % 2 == 1 { k / 2 } else { k / 2 - 1 };
            let mut pal = 10usize.pow(temp);
            pal += n - 1;
            print!("{pal}");
            if k % 2 == 1 {
                pal /= 10;
            }
            while pal != 0 {
                print!("{}", pal % 10);
                pal /= 10;
            }
            println!();
            return;
        }

        n -= amount;
    }
}
