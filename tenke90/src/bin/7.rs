use proconio::input;

fn main() {
    input! {n: usize, mut a: [i64; n], q: usize, b: [i64; q]};

    let mut t = vec![-0x3f3f3f3f3f3f3f3f];
    t.append(&mut a);
    t.push(0x3f3f3f3f3f3f3f3f);
    t.sort();

    for now in b {
        let mut l = 0;
        let mut r = t.len();
        while r - l > 1 {
            let m = (r + l) / 2;
            if t[m] > now {
                r = m;
            } else {
                l = m;
            }
        }

        println!("{}", std::cmp::min(t[r] - now, now - t[l]));
    }
}