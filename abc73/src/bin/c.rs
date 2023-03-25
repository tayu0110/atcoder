use proconio::input;

fn main() {
    input! {n: usize, mut a: [u32; n]}
    a.sort();

    let mut stack = vec![];
    for a in a {
        match stack.last() {
            Some(&l) if l == a => {
                stack.pop().unwrap();
            }
            _ => stack.push(a),
        }
    }

    println!("{}", stack.len())
}
