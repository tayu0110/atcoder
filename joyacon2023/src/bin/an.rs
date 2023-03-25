use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0;
    let mut board = vec![0; 4];
    for a in a {
        board[0] += 1;
        let mut new = vec![0; 4];
        for j in 0..4 {
            if j + a >= 4 {
                res += board[j];
            } else {
                new[j + a] += board[j];
            }
        }
        board = new;
    }

    println!("{}", res);
}
