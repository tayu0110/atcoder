use libc::{
    c_char, 
    scanf, printf
};

fn main() {
    let mut a = 0;
    let mut b = 0;
    unsafe {
        scanf("%d %d\0".as_ptr() as *const c_char, &mut a, &mut b);    
    }

    if (a * b) % 2 == 1 {
        unsafe {
            printf("Odd\n\0".as_ptr() as *const c_char);
        }
    } else {
        unsafe {
            printf("Even\n\0".as_ptr() as *const c_char);
        }
    }
}