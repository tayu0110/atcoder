use proconio::input;

fn main() {
    input! {n: usize, s: usize, a: [usize; n]}

    let mut memo = vec![false; s + 1];
    memo[0] = true;
    for a in a {
        for i in (0..s).rev() {
            if i + a <= s {
                memo[i + a] |= memo[i];
            }
        }
    }

    if memo[s] {
        println!("Yes")
    } else {
        println!("No")
    }
}
