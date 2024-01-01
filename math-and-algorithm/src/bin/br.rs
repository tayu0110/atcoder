use proconio::*;

const N: usize = 100;
const X: usize = 300;
const RES: [[u16; X + 1]; N + 1] = {
    let mut res = [[0; X + 1]; N + 1];
    let mut a = 1;
    while a <= N {
        let mut b = a + 1;
        while b <= N {
            let mut c = b + 1;
            while c <= N {
                let x = a + b + c;
                if x > X {
                    break;
                }
                res[c][x] += 1;
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }

    let mut n = 1;
    while n <= N {
        let mut x = 0;
        while x <= X {
            res[n][x] += res[n - 1][x];
            x += 1;
        }
        n += 1;
    }

    res
};

fn main() {
    input! {n: u8, x: u16}
    println!("{}", RES[n as usize][x as usize])
}
