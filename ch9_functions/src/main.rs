fn main() {
    println!("Hello, world!");

    functions_main_test();
    methods_test();
    closures_test();
    higher_order_functions_test();
    diverging_functions_test();
}

fn functions_main_test() {
    fizzbuzz_to(100);    
    
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs == 0
    }

    fn fizzbuzz(x: u32) {
        if is_divisible_by(x, 15) {
            println!("fizzbuzz...");
        } else if is_divisible_by(x, 3) {
            println!("fizz...");
        } else if is_divisible_by(x, 5) {
            println!("buzz...");
        } else {
            println!("{}", x);
        }
    }
    
    fn fizzbuzz_to(n: u32) {
        for x in 1..=n {
            fizzbuzz(x);
        }
    }  
}

fn methods_test() {
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn origin() -> Point {
            Point {
                x: 0.0,
                y: 0.0,
            }
        }

        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    } 

    impl Rectangle {
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;           
            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            ((x1- x2).abs() + (y1 - y2).abs()) * 2.0
        }

        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;
            self.p1.y += y;
            self.p2.y += y;
        }
    }

    struct Pair(Box<i32>, Box<i32>);
    impl Pair {
        fn destory(self) {
            let Pair(first, second) = self;
            println!("Destorying Pair({}, {})", first, second);
        }
    }

    //...main
    let rect = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("Rectangle perimeter: {}", rect.perimeter());
    println!("Rectangle area: {}", rect.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(8.0, 9.0),
    };
    square.translate(2.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destory();
}

fn closures_test() {
    //simple test
    {
        fn function(i: i32) -> i32 { i + 1 }
        let closures_annotated = |i: i32| -> i32 { i + 1 };
        let closures_inferred = |i| { i + 1 };

        let i = 1;
        println!("function: {}", function(i));
        println!("closures_annotated: {}", closures_annotated(i));
        println!("closures_inferred: {}", closures_inferred(i));

        let one = || { 1 };
        println!("closure returning one: {}", one());
    }
    capturing_test();
    as_input_parameters_test();
    type_anonymity_test();
    input_functions_test();
    examples_in_std_test();

    // &T / &mut T / T
    fn capturing_test() {
        use std::mem;
        
        //...&T
        println!("/** &T **/");
        let color = String::from("green");
        let print = || println!("color: {}", color);
        print();
        let re_imutable_borrow = &color;
        print();
        println!("re_imutable_borrow color: {}", re_imutable_borrow);
        let _color_moved = color;
        println!("color_moved: {}", _color_moved);

        println!("/** &mut T **/");
        let mut count = 0;
        let mut inc = || {
            count += 1;
            println!("count: {}", count);  
        };
        inc();
        inc();
        inc();
        let _mut_reborrow = &mut count;
        println!("mut_reborrow: {}", _mut_reborrow);

        println!("/** T **/");
        let movable = Box::new(3);
        let consume = || {
            println!("movable: {:?}", movable);
            mem::drop(movable);
        };
        consume();
        //consume(); //消耗了该变量，所以该闭包只能调用一次。

        let haystack = vec![1, 2, 3];
        let contains = move |needle| haystack.contains(needle);

        println!("{}", contains(&1));
        println!("{}", contains(&4));
        //println!("vec has {} elements.", haystack.len());
    }

    fn as_input_parameters_test() {
        /* 
        当以闭包作为输入参数时，必须指出闭包的完整类型，
        它是通过使用以下 trait 中的一种来指定的。其受限制程度按以下顺序递减：

        Fn：表示捕获方式为通过引用（&T）的闭包
        FnMut：表示捕获方式为通过可变引用（&mut T）的闭包
        FnOnce：表示捕获方式为通过值（T）的闭包   
        */
        fn apply<F>(f: F)
        where F: FnOnce() {
            f();
        }
        fn apply_to_3<F>(f: F) -> i32
        where F: Fn(i32) -> i32 {
            f(3)
        }

        use std::mem;
        let greeting = "hello";
        let mut farewell = "goodbye".to_owned();
        //引用捕获greeting, 值捕获farewell
        let diary = || {
            //...Fn
            println!("I said {}.", greeting);
            //...FnMut
            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            //...FnOnce
            mem::drop(farewell);
        };  

        apply(diary); //...闭包入参类型为 FnOnce，闭包 diary 为FnOnce变量
        let double = |x| x * 2;
        println!("3 doubled: {}", apply_to_3(double));        
    }

    fn type_anonymity_test() {
        
    }

    fn input_functions_test() {
        
    }

    fn examples_in_std_test() {
        //...Iterator::any

        //...searching through iterators
        
    }
}

fn higher_order_functions_test() {
    
}

fn diverging_functions_test() {
    
}