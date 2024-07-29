#![allow(dead_code)]
fn main() {
    println!("Hello, world!");

    if_else_test();
    loop_test();
    while_test();
    for_range_test();
    match_test();
    if_let_test();
    //let_else_test();
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
        let pair = (2, -2);
        println!("pair: {:?}", pair);
        match pair {
            (x, y) if x == y => println!("x == y"),
            (x, y) if x + y == 0 => println!("x + y == 0"),
            (x, _) if x % 2 == 1 => println!("x is odd number"),
            _ => println!("Nothing..."),
        }
    } 

    fn binding_test() {
        fn age() -> u32 {
            15
        }
        match age() {
            0 => println!("age is 0"),
            n @ 1..=12 => println!("I'm a child of age {:?}", n),
            n @ 13..=19 => println!("I'm a teen of age {:?}", n),
            n => println!("I'm an old person of age {:?}", n),
        }

        // 用绑定来“解构” enum 变体，例如 Option
        fn some_number() -> Option<u32> {
            Some(42)
        } 

        match some_number() {
            Some(n @ 42) => println!("The Answer: {}", n),
            Some(n) => println!("Incorrect number: {}", n),
            _ => (),
        }
    }
}

fn if_let_test() {
    //用 match 匹配枚举类型并不优雅
    let optional = Some(7);
    match optional {
        Some(i) => println!("hello, {}", i),
        _ => (),
    }
    
    //if let 要更简洁，并且允许指明数种失败情形下的选项：
    let number = Some(9);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } else {
        println!("Not matched...");
    }

    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Not Matched...");
    } else {
        println!("I don't like letters...");
    }

    //...match enum
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(val) = c {
        println!("c is {}", val);
    }

    //...fix
    let a = Foo::Bar;
    if let Foo::Bar = a {
        println!("a is foobar");
    }
}

fn while_let_test() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit...");
            optional = None;
        } else {
            println!("i is {}, try it again...", i);
            optional = Some(i + 1);
        }
    }
    /*
        if let 有可选的 else/else if 语句
        但 while let 没有
    */
}
