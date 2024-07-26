#![allow(dead_code)]
fn main() {
    println!("Hello, world!");

    casting();
    literals();
    inference();
    aliasing();
}

fn casting() {
    //...as
    let a = 3.14_f32;
    let b = 666;

    let a_v0 = a as u8;
    let b_v0 = b as f32;

    println!("1000 as u8 is {}", 1000 as u16);
}

fn literals() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn inference() {
    let elem = 6u8;
    let mut vec: Vec<u8> = Vec::new();
    //vec.push(elem);
    println!("{:?}", vec);
}

fn aliasing() {
    //...
}
