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

    //todo!();

    capturing_test();
    as_input_parameters_test();
    type_anonymity_test();
    input_functions_test();
    examples_in_std_test();

    fn capturing_test() {
        
    }

    fn as_input_parameters_test() {
        
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