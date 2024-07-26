fn main() {
    println!("Hello, world!");

    let x = 5_u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        x_cube + x_squared + x
    };
    let z = {
        2 * x;
    };

    println!("x is {}", x); //5
    println!("y is {}", y); //155
    println!("z is {:?}", z); //()
}
