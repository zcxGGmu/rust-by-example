#![crate_name = "ch1_hello_world"]
//#![warn(dead_code)]

use ch1_hello_world::add_one;

fn main() {
    println!("Hello, world!");
    println!("I am a Rustacean!");
        
    comments();

    println!("last_valuie: {}", add_one(9));

    println!("==============================>>>");

    formatted_print();
}

fn comments() {
    // line comments
    println!("line comments.");    
    /* block comments */
    println!("block comments.");
    
    let x = 5 + /* 90 + */ 5;
    println!("x = {}", x);
}

fn formatted_print() {

}

fn debug() {
    
}

fn display() {
    
}

fn fomatting() {
    
}

