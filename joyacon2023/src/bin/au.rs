use proconio::input;

fn main() {
    input! {mut x: i32, mut y: i32, mut z: i32}

    if x < 0 {
        x = -x;
        y = -y;
        z = -z;
    }

    if y < 0 || x < y {
        println!("{}", x);
    } else if z < 0 {
        println!("{}", z.abs() * 2 + x);
    } else if y < z {
        println!("-1");
    } else {
        println!("{}", x);
    }
}
