//ch9 pg 150 of Rust Essentials
#![feature(asm)]

fn subtract (a: i32, b: i32) -> i32{
    let sub: i32;

    unsafe{
        asm!("sub $2, $1; mov $1, $0"
            : "=r"(sub)
            : "r"(a), "r"(b) 
        );
    }
    sub

}

fn main(){
    println!("{}", subtract(42, 7));
}

