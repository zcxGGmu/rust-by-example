#![allow(dead_code)]

use std::fmt::{self, Formatter, Display};
use std::mem;

fn main() {
    println!("Hello, world!");
    primitives();
}

fn primitives() {
    let mut logical: bool = true;
    let a_float: f32 = 1.0;
    let b_integer = 5u32;
    
    let def_float = 3.0;
    let def_integer = 7;

    // infer by context
    let mut inferred_type = 12;
    inferred_type = 212175875i64;
    
    logical = false;
    let logical = 12i64; // shadow

    literals_operators();
    tuples();
    arrays_slices();
}

fn literals_operators() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1[i32] - 2 = {}", 1i32 - 2);
    //println!("1[u32] - 2 = {}", 1u32 - 2); //error: overflow
}

fn tuples() {
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;
        (boolean, integer)
    }

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);
    
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    
    println!("tuple_of_tuples: {:?}", tuple_of_tuples);
    
    // 但很长的元组无法打印
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("too long tuple: {:?}", too_long_tuple);
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);
    
    impl Display for Matrix {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "( {} {} )\n( {} {} )",
                self.0, self.1, self.2, self.3)
        }
    }

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);

    println!();
    fn transpose(m: Matrix) -> Matrix {
        Matrix(m.0, m.2, m.1, m.3)
    }
    println!("Transpose:\n{}", transpose(matrix));
}

fn arrays_slices() {
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slices: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    let arr_1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_2: [i32; 500] = [0; 500];
    
    println!("array occupies {} bytes", mem::size_of_val(&arr_1));
    
    analyze_slice(&arr_1);
    analyze_slice(&arr_1[1..4]);
}
