#![allow(dead_code)]

fn main() {
    println!("Hello, world!");

    var_binding();    
    mutability();
    scope_shadowing();
    declare_first();
    freezing();
}

fn var_binding() {
    let a_int = 6u32;
    let b_bool = true;
    let unit = ();

    let a_copy = a_int;
    let _unused_var = 666u32;    
}

fn mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    mutable_binding += 56;
    println!("mut_var: {}", mutable_binding);
}

fn scope_shadowing() {
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        let long_lived_binding = 10;
        println!("inner long: {}", long_lived_binding);
    }
    //println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);
    let long_lived_binding = "abc".to_owned();
    println!("shadow long: {}", long_lived_binding);
}

fn declare_first() {
    let a;
    {
        let x = 2;
        a= x * x;
    }
    println!("a: {}", a);
}

fn freezing() {
    let mut _mutable_int = 66i32;
    {
        let mut _mutable_int = _mutable_int;
        _mutable_int = 999;
        println!("inner var: {}", _mutable_int); //999
    }
    _mutable_int = 1000;
    println!("outer var: {}", _mutable_int); //1000

    let a_test = 777;
    println!("a_test_1st: {}", a_test);
    let a_test = 888;
    println!("a_test_2nd: {}", a_test);
}
