use proconio::input;

fn main() {
    input! {n: usize, l: usize, r: usize};

    let (mut l, mut r) = (l-1, r-1);
    let mut x = 1;
    let mut y = 2;
    while l > 0 {
        if n - x <= l {
            l -= n - x;
            x += 1;
            y = x + 1;
        } else {
            y = x + l + 1;
            l = 0;
        }
    }
    let mut tx = 1;
    let mut ty = 2;
    while r > 0 {
        if n - tx <= r {
            r -= n - tx;
            tx += 1;
            ty = tx + 1;
        } else {
            ty = tx + r + 1;
            r = 0;
        }
    }
    // eprintln!("x: {}, y: {}, tx: {}, ty: {}", x, y, tx, ty);
    let mut res = vec![];
    for i in 1..x {
        res.push(i);
    }
    let mut buf = vec![];
    for i in x+1..n+1 {
        if i == y {
            buf.push(x);
        }
        buf.push(i);
    }
    while res.len() < tx - 1 {
        res.push(buf.pop().unwrap());
    }
    ty -= tx;
    for i in 1..ty+1 {
        buf.swap(0, i);
    }
    for v in buf {
        res.push(v);
    }
    for (i, v) in res.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", *v);
    }
    println!("");
}
