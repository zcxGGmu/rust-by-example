#![crate_name = "ch1_hello_world"]
#![warn(dead_code)]

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
    println!("{0}, this is {1}. {1}, this is {0}", "quan", "zhou");
    println!("my name is {0} {1}", "Quan", "Zhou");

    println!("{system}-{version}-{release}",
        system = "LINUX",
        version = "6.11",
        release = "rc1");

    println!("{} of {:b} people", 1, 2);

    println!("{number:>width$}", number=1, width=6);
    println!("{number:0width$}", number=1, width=6);

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);
    println!("this struct `{:?}` won't print...", Structure(666));
    
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    /* Debug */
    println!();
    debug();

    println!();
    display();

    println!();
    formatting();
}

fn debug() {
    struct UnPrintable(i32);
    #[derive(Debug)]
    struct DebugPrintable(i32);
    #[derive(Debug)]
    struct Nested(DebugPrintable);

    println!("my name is {0:?} {1:?}. hello, {rust:?}",
        "Quan", "Zhou", rust="rust");
    
    println!("Now {:?} will print!", DebugPrintable(100));
    println!("Now {:?} will print!", Nested(DebugPrintable(200)));
    println!("Now {:#?} will print!", DebugPrintable(100));
    println!("Now {:#?} will print!", Nested(DebugPrintable(200)));
}

use std::fmt::{self, Formatter, Display};
fn display() {
    /*
        因为没有一种合适的样式适用于所有类型，标准库也并不擅自规定一种样式。
        对于 Vec<T> 或其他任意泛型容器（generic container），fmt::Display 都没有实现。
        因此在这些泛型的情况下要用 fmt::Debug
    */
    #[derive(Debug)]
    struct MinMax(i64, i64);
    
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }
    
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 7);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let point = Point2D { x: 1.1, y: 2.2 };
    println!("Display: {}", point);
    println!("Deubg: {:?}", point);
    
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{0} + {1}i", self.real, self.imag)
        }
    }

    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    println!();
    display_list();
}

fn display_list() {
    #[derive(Debug)]
    struct List(Vec<i32>);
    
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            
            for (idx, v) in vec.iter().enumerate() {
                if idx != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", idx, v)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("Display: {}", v);
    println!("Debug: {:?}", v);
}

fn formatting() {
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(f, "{}: {:.3}*{} {:.3}*{}",
                    self.name, self.lat.abs(), lat_c,
                    self.lon.abs(), lon_c)
        }
    }

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }
    impl Display for Color {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}",
                self.red, self.green, self.blue)
        }
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    println!();
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:?}", *color)
    }

    println!();
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color)
    }
}

