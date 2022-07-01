use proconio::input;

fn main() {
    input! {x: i64, a: i64, d: i64, n: i64};

    if d >= 0 {
        let mut l = 0i64;
        let mut r = n;
    
        while r - l > 1 {
            let m = (r + l) / 2;
            let now = a + d * m;
    
            if now < x {
                l = m;
            } else {
                r = m;
            }
        }
    
        if r == n {
            println!("{}", (x - (a + d * l)).abs());
        } else {
            println!("{}", std::cmp::min((x - (a + d * l)).abs(), ((a + d * r) - x).abs()));
        }
    } else {
        let mut l = 0i64;
        let mut r = n;

        while r - l > 1 {
            let m = (r + l) / 2;
            let now = a + d * m;

            if now < x {
                r = m;
            } else {
                l = m;
            }
        }

        if r == n {
            println!("{}", ((a + d * l) - x).abs());
        } else {
            println!("{}", std::cmp::min(((a + d * l) - x).abs(), (x - (a + d * r)).abs()));
        }
    }
}