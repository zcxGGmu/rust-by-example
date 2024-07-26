#![allow(dead_code)]

use std::convert::From;
use std::convert::{TryFrom, TryInto};

fn main() {
    println!("Hello, world!");

    from_into();
    tryfrom_tryinto();
    to_from_strings();
}

fn from_into() {
    println!("/** from: **/");
    let my_str = "hello";
    let my_string = String::from(my_str);
    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Self {
                value: item
            }
        }
    }
    let num = Number::from(30);
    println!("My number_from is {:?}", num);
    println!("");
    println!("/** into: **/");
    let int = 5_i32;
    let num: Number = int.into();
    println!("My number_into is {:?}", num);
}

fn tryfrom_tryinto() {
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);
    
    impl TryFrom<i32> for EvenNumber {
        type Error = ();
        
        fn try_from(item: i32) -> Result<Self, Self::Error> {
            if item % 2 == 0 {
                Ok(EvenNumber(item))
            } else {
                Err(())
            }
        }
    }
    //TryFrom
    assert_eq!(EvenNumber::try_from(99), Err(()));
    assert_eq!(EvenNumber::try_from(88), Ok(EvenNumber(88)));
    //TryInto
    let res: Result<EvenNumber, ()> = 77_i32.try_into();
    assert_eq!(res, Err(()));
    let res: Result<EvenNumber, ()> = 66_i32.try_into();
    assert_eq!(res, Ok(EvenNumber(66)));
}

fn to_from_strings() {
    //要把任何类型转换成 String，只需要实现那个类型的 ToString trait
    //然而不要直接这么做，您应该实现fmt::Display trait
    //它会自动提供 ToString，并且还可以用来打印类型
    use std::fmt;
    struct Circle {
        radius: i32,
    }
    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }
    let circle = Circle { radius: 66 };
    println!("normal-display => {}", circle);
    println!("print-string => {}", circle.to_string());

    //...parse
    let parsed: i32 = "700".parse().unwrap();
    let turbo_parsed = "800".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
