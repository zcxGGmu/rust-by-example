fn main() {
    println!("Hello, world!");
    
    error_handling_test();
}

fn error_handling_test() {
    panic_test();
    abort_unwind_test();
    option_unwrap_test();
    result_test();
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
        
    }
}

fn result_test() {
    
}

fn multiple_error_types_test() {
    
}

fn iterating_over_results() {
    
}
