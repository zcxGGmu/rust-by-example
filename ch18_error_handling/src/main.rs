use std::fmt::Result;

fn main() {
    println!("Hello, world!");
    
    error_handling_test();
}

fn error_handling_test() {
    panic_test();
    abort_unwind_test();
    option_unwrap_test();
    //result_test();
    multiple_error_types_test();
    iterating_over_results();
}

fn panic_test() {
    fn give_princess(gift: &str) {
        if gift == "snake" {
            panic!("game over...");
        }
        println!("everything is ok...");
    }

    give_princess("teddy");
    //give_princess("snake");
}

fn abort_unwind_test() {
    //...
}

fn option_unwrap_test() {
    /*
        enum Option<T> {
            Some(val: T),
            None,
        }
        这些选项可以通过 match 显式地处理，或使用 unwrap 隐式地处理
        隐式处理要么返回 Some 内部的元素，要么就 panic!
    */
    //...显式使用 `match` 来处理
    fn give_commoner(gift: Option<&str>) {
        match gift {
            Some("snake") => println!("this is a snake..."),
            Some(inner) => println!("nice, this is a {:?}", inner),
            None => println!("no gift..."),
        }
    }
    //...隐式使用 `unwrap` 来处理
    fn give_princess(gift: Option<&str>) {
        let inner = gift.unwrap();
        if inner == "snake" {
            panic!("this is a snake...");
        } else {
            println!("i love the gift: {}", inner);
        }        
    }

    let food = Some("chicken");
    let snake = Some("snake");
    let null =  None;
    give_commoner(food);
    give_commoner(snake);
    give_commoner(null);

    let bird = Some("robin");
    let nothing: Option<&str> = None;
    give_princess(bird);
    //give_princess(nothing);
    {
        //...使用 ? 解开 Option
        fn next_birthday(current_age: Option<u8>) -> Option<String> {
            // 如果 `current_age` 是 `None`，这将返回 `None`
            // 如果 `current_age` 是 `Some`，内部的 `u8` 将赋值给 `next_age`
            let next_age = current_age?;
            Some(format!("next year i will be {}", next_age))
        }

        struct Person {
            job: Option<Job>,
        }
        #[derive(Clone, Copy)]
        struct Job {
            phone_number: Option<PhoneNumber>,
        }
        #[derive(Clone, Copy)]
        struct PhoneNumber {
            area_code: Option<u8>,
            number: u32,
        }
        impl Person {
            fn work_phone_area_code(&self) -> Option<u8> {
                //...没有 ？ 此处需要多次 match 嵌套
                self.job?.phone_number?.area_code                
            }
        }
        
        //...test
        let p = Person {
            job: Some(Job {
               phone_number: Some(PhoneNumber {
                    area_code: Some(66),
                    number: 110,
               }),
            }),
        };
        assert_eq!(p.work_phone_area_code(), Some(66));
        println!("p.work_phone_area_code: {}", p.work_phone_area_code().unwrap());
    }
    { 
        //...组合算子 map
        #[derive(Debug)] enum Food { Apple, Carrot, Potato }
        #[derive(Debug)] struct Peeled(Food);
        #[derive(Debug)] struct  Chopped(Food);
        #[derive(Debug)] struct  Cooked(Food);

        fn peel(food: Option<Food>) -> Option<Peeled> {
            match food {
                Some(food) => Some(Peeled(food)),
                None => None,
            }
        }

        fn chop(peeded: Option<Peeled>) -> Option<Chopped> {
            match peeded {
                Some(Peeled(f)) => Some(Chopped(f)),
                None => None,
            }
        }

        fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
            chopped.map(|Chopped(f)| Cooked(f))
        }

        fn process(f: Option<Food>) -> Option<Cooked> {
            f.map(|f| Peeled(f))
             .map(|Peeled(f)| Chopped(f))
             .map(|Chopped(f)| Cooked(f))
        }

        fn eat(food: Option<Cooked>) {
            match food {
                Some(food) => println!("Mmm, i love {:?}", food),
                None               => println!("Oh no, it wasn't edible"),
            }
        }

        let apple = Some(Food::Apple);
        let carrot = Some(Food::Carrot);
        let potato = None;

        let cooked_apple = cook(chop(peel(apple)));
        let cooked_carrot = cook(chop(peel(carrot)));
        let cooked_potato = process(potato);

        eat(cooked_apple);
        eat(cooked_carrot);
        eat(cooked_potato);
    }
    {
        //...组合算子 and_then
        #![allow(dead_code)]

        #[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
        #[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

        //no Sushi
        fn have_ingredients(food: Food) -> Option<Food> {
            match food {
                Food::Sushi => None,
                _           => Some(food),
            }
        }

        //no CordonBleu
        fn have_recipe(food: Food) -> Option<Food> {
            match food {
                Food::CordonBleu => None,
                _                => Some(food),
            }
        }

        fn cookable_v1(food: Food) -> Option<Food> {
            match have_ingredients(food) {
                Some(food) => match have_recipe(food) {
                    Some(food) => Some(food),
                    None             => None,
                },
                None => None,
            }
        }

        fn cookable_v2(food: Food) -> Option<Food> {
            have_ingredients(food).and_then(have_recipe)
        }

        fn eat(food: Food, day: Day) {
            match cookable_v2(food) {
                Some(food) => println!("day: {:?}, food: {:?}", day, food),
                None             => println!("day: {:?}, we don't eat anything...", day),
            }
        }
        
        let (a, b, c) = (Food::CordonBleu, Food::Steak, Food::Sushi);
        eat(a, Day::Monday);
        eat(b, Day::Tuesday);
        eat(c, Day::Wednesday);
    }
}

fn result_test() {
    {
        // Result<T, E> => Ok<T>, Err<E>
        // Option<T> => Some<T>, None
        fn multiply(num_str_1: &str, num_str_2: &str) -> i32 {
            let num_1 = num_str_1.parse::<i32>().unwrap();
            let num_2 = num_str_2.parse::<i32>().unwrap();
            num_1 * num_2
        }

        let twenty = multiply("10", "2");
        println!("twenty == {}", twenty);
        
        //let tt = multiply("t", "2");
        //println!("this is a {}", tt);
    }
    {
        // 常规写法，分别处理，提前返回
        /*
        use std::num::ParseIntError;
        fn multiply(num_1_str: &str, num_2_str: &str) -> Result<i32, ParseIntError> {
            let num_1 = match num_1_str.parse::<i32>() {
                Ok(num_1) => num_1,
                Err(e) => return Err(e),  
            };

            let num_2 = match num_2_str.parse::<i32>() {
                Ok(num_2) => num_2,
                Err(e) => return Err(e),
            };

            Ok(num_1 * num_2)
        }
        */
    }
    {
        // ? 可以提前返回; try!是 ? 出现前的写法
        /*
        use std::num::ParseIntError;
        fn multiply(num_1_str: &str, num_2_str: &str) -> Result<i32, ParseIntError> {
            let num_1 = num_1_str.parse::<i32>()?;
            let num_2 = num_2_str.parse::<i32>()?;
            Ok(num_1 * num_2)
        }
        */
    }
}

use std::num::ParseIntError;
use std::str::ParseBoolError;
fn multiple_error_types_test() {
    // 处理 混合错误类型
    {
        fn double_first(vec: Vec<&str>) -> i32 {
            let first = vec.first().unwrap();
            2 * first.parse::<i32>().unwrap()
        }
        let numbers = vec!["42", "93", "18"];
        //let empty = vec![];
        let strings = vec!["tofu", "93", "18"];
        //println!("the first doubled is {}", double_first(numbers));
        //println!("the first doubled is {}", double_first(empty));
        //println!("the first doubled is {}", double_first(strings));
    }
    {
        //相互包含
        /*
        fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
            vec.first().map(|first| {
                first.parse::<i32>().map(|n| n * 2)
            })
        }
        */
    }
    {
        use std::error;
        use std::fmt;

        type Result<T> = std::result::Result<T, DoubleError>;
        
        #[derive(Debug, Clone)]
        struct DoubleError;

        impl fmt::Display for DoubleError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "invalid first item to double")
            }
        }

        fn double_first(vec: Vec<&str>) -> Result<i32> {
            vec.first()
                .ok_or(DoubleError)
                .and_then(|s| {
                    s.parse::<i32>()
                        .map_err(|_| DoubleError)
                        .map(|i| 2 * i)
                })
        }

        fn print(result: Result<i32>) {
            match result {
                Ok(n) => println!("first_doubled: {}", n),
                Err(e) => println!("Error: {}", e),
            }
        }
        
        let numbers = vec!["11", "22", "33"];
        //let empty = vec![];
        let strings = vec!["tofu", "93", "18"];
        
        print(double_first(numbers));
        //print(double_first(empty));
        //print(double_first(strings));        
    }
    {
        // 包裹错误
        use std::error;
        use std::num::ParseIntError;
        use std::fmt;

        type Result<T> = std::result::Result<T, DoubleError>;
        
        #[derive(Debug)]
        enum DoubleError {
            EmptyVec,
            Parse(ParseIntError),
        }

        impl fmt::Display for DoubleError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    DoubleError::EmptyVec =>
                        write!(f, "this is a empty_vec..."),
                    DoubleError::Parse(ref e) => e.fmt(f),
                }
            }
        }
        
    }
}

fn iterating_over_results() {
    // 遍历 Result
    
}
