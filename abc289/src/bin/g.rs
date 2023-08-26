use proconio::input;

fn main() {
    input! {n: usize, m: usize, mut b: [usize; n], c: [usize; m]}
    b.sort();
    b.reverse();

    let mut convex = vec![(0, 1, b[0])];
    for i in 1..n {
        let mut x;
        let a = i + 1;
        let b = a * b[i];
        loop {
            let (x_prev, a_prev, b_prev) = convex.pop().unwrap();
            x = (b_prev + a - (b_prev + a).min(b + a_prev)) / (a - a_prev);
            if x_prev < x {
                convex.push((x_prev, a_prev, b_prev));
                break;
            }
            if convex.is_empty() {
                break;
            }
        }

        convex.push((x, a, b));
    }

    for c in c {
        let (mut l, mut r) = (0, convex.len());
        while r - l > 1 {
            let m = (r + l) / 2;
            if convex[m].0 < c + 1 {
                l = m;
            } else {
                r = m;
            }
        }
        let (_, a, b) = convex[l];
        println!("{}", a * c + b);
    }
}
