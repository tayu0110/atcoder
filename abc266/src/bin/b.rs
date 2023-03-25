use proconio::input;

fn main() {
    input! {n: i64};

    println!("{}", (n % 998244353 + 998244353) % 998244353);
}