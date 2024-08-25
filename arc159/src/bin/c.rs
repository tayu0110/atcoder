use proconio::*;
// use rand::{thread_rng, Rng};

fn main() {
    // let mut rng = thread_rng();
    // let n: usize = rng.gen_range(2, 51);
    // let mut a = (0..n).map(|_| rng.gen_range(1, 51)).collect::<Vec<usize>>();
    // eprintln!("a: {:?}", a);
    input! {n: usize, mut a: [usize; n]}

    if a.iter().sum::<usize>() % n != 0 {
        println!("No");
        return;
    }

    println!("Yes");

    let _av = a.iter().sum::<usize>() / n;
    // let mut res = vec![];
    // loop {}
    // println!("{}", res.len());
    // for v in res {
    //     println!("{}", v.iter().map(|a| a + 1).join(" "));
    // }
}
