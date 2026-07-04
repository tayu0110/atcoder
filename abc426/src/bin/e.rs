use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {
            mut tsx: i32, mut tsy: i32, mut tgx: i32, mut tgy: i32,
            mut asx: i32, mut asy: i32, mut agx: i32, mut agy: i32,
        }

        let minx = tsx.min(tgx).min(asx).min(agx);
        let miny = tsy.min(tgy).min(asy).min(agy);
    }
}
