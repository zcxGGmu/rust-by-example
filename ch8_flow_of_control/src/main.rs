#![allow(dead_code)]
fn main() {
    println!("Hello, world!");

    if_else_test();
    loop_test();
    while_test();
    for_range_test();
    match_test();
    if_let_test();
    let_else_test();
    while_let_test();
}

fn if_else_test() {
    let n = 66;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let n_changed = if n < 10 && n > -10 {
        println!(", and is small number, increase ten-fold.");
        10 * n
    } else if n <= -10 {
        println!(", and is samller than -10, improve to 0.");
        n - n
    } else {
        println!(", and is big number, half the number.");
        n / 2
    };

    println!("{} => {}", n, n_changed);
}

fn loop_test() {
    //...loop/break/continue
    let mut count = 0_u32;
    println!("Let's count until infinity!");

    loop {
        count += 1;
        if count == 3 {
            println!("three, ...continue");
            continue;
        }
        println!("current number: {}", count);
        if count == 5 {
            println!("Ok, that's enough!");
            break;
        }
    }

    nesting_labels_test();
    returning_from_loops();

    fn nesting_labels_test() {
        'outer: loop {
            println!("Entered the outer loop");
            'inner: loop {
                println!("Entered the inner loop");
                //break;
                break 'outer;
            }
            println!("This point will never be reached!");
        }
        println!("Exited the outer loop");
    }

    fn returning_from_loops() {
        let mut count = 0;
        let res = loop {
            count += 1;
            if count == 66 {
                break (count - 6) * 10
            }
        };
        assert_eq!(res, 600);
    }
}

fn while_test() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

fn for_range_test() {
    for mut n in 102..=153 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
    println!();
    // for and iterator
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("hello, rust!"),
            _ => println!("hello, world!"),
        }
    }
    println!();
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("into_iter: hello, ferris!"),
            _ => println!("hello, {}", name),
        }
    }
    println!();
    let mut names = vec!["AAA", "BBB", "CCC"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "AAA" => "hello, AAA",
            _ => "hello, other",
        }
    }
    println!("{:?}", names);

}

fn match_test() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,  
    };
    println!("{} => {}", boolean, binary);

    destructuring_test();
    guards_test();
    binding_test();

    fn destructuring_test() {
        println!("/** tuple: **/");
        let triple = (3, -2, 3);
        match triple {
            (0, y, z) => println!("first is `0`, `y` is {:?}, `z` is {:?}", y, z),
            (1, ..) => println!("first is `1`, and other whatever"),
            _ => println!("it doesn't matter what they are"),
        } 
        println!();
        println!("/** enum: **/");
        enum Color {
            Red,
            Blue,
            Green,
            RGB(u32, u32, u32),
        }
        let color = Color::RGB(122, 17, 40);
        println!("What color is it?");
        match color {
            Color::Red => println!("The color is Red!"),
            Color::Blue => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) => {
                println!("Red: {}, green: {}, blue: {}", r, g, b);
            }
        }
        println!();
        println!("/** pointer/reference: **/");
        let reference = &666;
        match reference {
            &val => println!("val: {}", val),
        }
        match *reference {
            val => println!("val: {}", val),
        }
        let _not_reference = 3;
        let ref _is_a_reference = 3;
        let val = 5;
        let mut val_mut = 6;
        match val {
            ref r => println!("inmutable val: {}", r),
        }
        match val_mut {
            ref mut r => {
                *r += 100;
                println!("mutable val: {}", r);   
            }
        }
        println!();
        println!("/** structure: **/");
        #[derive(Debug)]
        struct Foo {
            x: (u32, u32),
            y: u32,
        }
        let foo = Foo {
            x: (1, 2),
            y: 3,
        };
        let Foo {
            x: (a, b),
            y: c,
        } = foo;
        println!("{:?}", foo);
        println!("foo.x: {:?}, foo.y: {:?}", (a, b), c);
        let Foo { y, .. } = foo;
        println!("foo.y = {:?}", y);
    }

    fn guards_test() {
        
    } 

    fn binding_test() {
        
    }
}

fn if_let_test() {
    
}

fn let_else_test() {
    
}

fn while_let_test() {
    
}
