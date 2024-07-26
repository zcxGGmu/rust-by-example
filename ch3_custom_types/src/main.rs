#![allow(dead_code)]

use std::fs::OpenOptions;

fn main() {
    println!("Hello, world!");
    
    //structures();
    enums();
    constants();
}

fn structures() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Person {
        name: String,
        age: u8,
    }
    struct Unit;
    struct Pair(i32, i32);

    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let name = String::from("zcx");
    let age = 27;
    let zcx = Person { name, age };
    
    println!("{:?}", zcx);

    let point = Point { x: 1.1, y: 2.2 };
    println!("({}, {})", point.x, point.y);

    // ..point 必须放在最后一个参数位置
    let bottom_right = Point { x: 6.6, ..point };
    
    let Point { x: left_edge, y: top_edge } = point;
    let _rectangle = Rectangle {
        top_left: Point {  x: left_edge, y: top_edge },
        bottom_right,
    };
    
    // 实例化一个单元结构体
    let _unit = Unit;

    let pair_tuple = Pair(1, 2);
    let Pair(left_t, right_t) = pair_tuple;
    println!("{}, {}", left_t, right_t);
    
    println!("/** test_1: **/");
    fn rect_area(rect: Rectangle) -> f32 {
        let Rectangle { top_left: tl, bottom_right: br } = rect;
        let width = (br.x - tl.x).abs();
        let height = (tl.y - br.y).abs();
        width * height
    }
    let rect = Rectangle {
        top_left: Point { x: 1.0, y: 6.0 },
        bottom_right: Point { x: 4.0, y: 2.0 },
    };
    /* width:3, height:4, area:12 */
    println!("width:{}, height:{}, area:{}", 
        (rect.bottom_right.x - rect.top_left.x).abs(),
        (rect.top_left.y - rect.bottom_right.y).abs(),
        rect_area(rect)
    );
    println!("/** test_2: **/");
    fn square(p: Point, len: f32) -> Rectangle {
        let p_x = p.x;
        let p_y = p.y;
        Rectangle {
            top_left: p,
            bottom_right: Point {
                x: p_x + len,
                y: p_y - len,
            },
        }
    }
    let p = Point {
        x: 1.1,
        y: 6.6,
    };
    let len = 2.0;
    let rect_new = square(p, len);
    println!("{:?}", rect_new);
}

fn enums() {
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 }
    }
    
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            WebEvent::KeyPress(c) => println!("pressed \'{}\'", c),
            WebEvent::Paste(s) => println!("pasted \"{}\"", s),
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}", x, y);
            },
        }
    }

    //print
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("hello-rust".to_owned());
    let click = WebEvent::Click { x: 66, y: 88 };
    let loaded = WebEvent::PageLoad;
    let unloaded = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(loaded);
    inspect(unloaded);

    //...
    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
    let add_ops = Operations::Add;
    let sub_ops = Operations::Subtract;
    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }
    let x = 100;
    let y = 300;
    println!("add({}, {}): {}", x, y, add_ops.run(x, y));
    println!("sub({}, {}): {}", x, y, sub_ops.run(x, y));

    /* use */


    /* c-style enum */


    /* list impl by enum */

    
}

fn constants() {

}