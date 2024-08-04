//use std::intrinsics::mir::Checked;

fn main() {
    println!("Hello, world!");

    std_lib_test();
}

fn std_lib_test() {
    //...

    box_stack_heap_test();
    vectors_test();
    strings_test();
    option_test();
    result_test();
    panic_test();
    hash_map_test();
    rc_test();
    arc_test();    
}

fn box_stack_heap_test() {
    use std::mem;

    //#[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }    

    struct Rectangle {
        p1: Point,
        p2: Point,
    } 

    fn origin() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
        }
    }

    fn boxed_origin() -> Box<Point> {
        Box::new(origin())
    }

    {
        let point_stack = origin();
        let rectangle_stack = Rectangle {
            p1: origin(),
            p2: Point { x: 0.1, y: 0.2 },
        };
        
        let point_heap = boxed_origin();
        let point_heap_nested = Box::new(boxed_origin());
        let rectangle_heap = Box::new(Rectangle {
            p1: origin(),
            p2: origin(), 
        });

        /*
            point_stack: 16
            rectangle_stack: 32
            point_heap: 8
            unref point_heap: 16
            point_heap_nested: 8
            rectangle_heap: 8
         */
        println!("point_stack: {}", mem::size_of_val(&point_stack));
        println!("rectangle_stack: {}", mem::size_of_val(&rectangle_stack));
        println!("point_heap: {}", mem::size_of_val(&point_heap)); 
        println!("unref point_heap: {}", mem::size_of_val(&*point_heap));
        println!("point_heap_nested: {}", mem::size_of_val(&point_heap_nested));
        println!("rectangle_heap: {}", mem::size_of_val(&rectangle_heap));              
    }
}

fn vectors_test() {
    let vec_collected: Vec<i32> = (0..=10).collect();
    println!("vec_collected: {:?}", vec_collected);

    let mut vec = vec![1, 2, 3, 4, 5];
    vec.push(6);
    println!("vec_size: {}", vec.len());
    println!("vec: {:?}", vec);
    println!("the last element of vec: {:?}", vec.pop()); 
    println!("vec: {:?}", vec);

    //for x in &vec {
    //    println!("vec[{}] = {}", i, vec[i]);
    //}

    for (i, x) in vec.iter().enumerate() {
        println!("vec[{:?}] = {:?}", i, x);
    }
    
    for x in &vec {
        println!("the element of vec: {:?}", x);
    }

    for x in vec.iter_mut() {
        *x *= 10;
    }
    println!("vec: {:?}", vec);
}

fn strings_test() {
    // String <=> Vec<u8>
    // &str <=> &[u8]
    let str_global: &'static str = "this is a string with global lifetime global lifetime";
    println!("strs_global: {}", str_global);

    //...逆序迭代
    for word in str_global.split_whitespace().rev() {
        println!("rev => strs_global: {}", word);
    }
    
    //...复制字符到一个 vector，排序并移除重复值
    let mut chars: Vec<char> = str_global.chars().collect();
    chars.sort();
    chars.dedup();
    println!("after sorted & removed: {:?}", chars);

    //...将 Vec<char> 转化为 String 类型
    let mut str_new = String::new();
    for c in chars {
        str_new.push(c);
        str_new.push_str(", ");
    }
    println!("str_new: {}", str_new);

    //...字符串裁剪，移除首尾匹配的字符
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = str_new.trim_matches(chars_to_trim);
    println!("after trimmed_str: {}", trimmed_str);

    //...堆，分配一个字符串，匹配并替换字符串 "dog"=>"cat"
    let str_heap = String::from("i like dogs");
    let str_heap_replaced = str_heap.replace("dog", "cat");
    println!("str_heap: {}", str_heap);
    println!("str_heap_replaced: {}", str_heap_replaced);
    {
        //转义字符: raw-string
        let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        println!("{}", raw_str);

        // 如果你要在原始字符串中写引号，请在两边加一对 #
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);

        // 如果字符串中需要写 "#，那就在定界符中使用更多的 #。
        // 可使用的 # 的数目没有限制。
        let longer_delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", longer_delimiter);
    }
    {
        //&str 和 String 都必须是合法的 UTF-8 序列
        //想要非 UTF-8 字符串，或字节数组，请使用字节串（byte string）
        let string_byte: &[u8; 20] = b"this is a stringbyte";
        println!("string_byte: {:?}", string_byte);

        let string_byte_raw = br"\u{211D} is not escaped here";
        println!("string_byte_raw: {:?}", string_byte_raw);

        //...把字节串转换为 &str 可能失败
        if let Ok(my_str) = std::str::from_utf8(string_byte_raw) {
            println!("string_raw into utf8: {:?}", my_str);
        }

        // 字节串可以不使用 UTF-8 编码
        let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82";
        // 但这样的话它们就无法转换成 &str 了
        match std::str::from_utf8(shift_jis) {
            Ok(my_str) => println!("Conversion ok: {}", my_str),
            Err(e) => println!("Conversion err: {:?}", e),
        }
    }
}

fn option_test() {
    //...有时候想要捕捉到程序某部分的失败信息，而不是调用 panic!
    fn check_division(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            None
        } else {
            Some(dividend / divisor)
        }
    }

    fn try_division(dividend: i32, divisor: i32) {
        match check_division(dividend, divisor) {
            None => println!("{}/{} failed...", dividend, divisor),
            Some(res) => println!("{}/{} = {}", dividend, divisor, res),
        }
    }

    try_division(4, 2);
    try_division(1, 0);

    let _none: Option<i32> = None;
    let _none_1 = None::<i32>;
    
    let _some_float = Some(0_f32);
    println!("{:?} unwraps to {:?}", _some_float, _some_float.unwrap());
    // None 无法被 unwrap
}

fn result_test() {
    // 有时要强调为什么一个操作会失败。为做到这点，我们提供了 Result 枚举类型
    // 检查 div/sqrt/ln
    mod checked {
        #[derive(Debug)]
        pub enum MathError {
            DivisionByZero,
            NegativeLogarithm,
            NegativeSquareRoot,
        }

        pub type MathResult = Result<f64, MathError>;

        pub fn div(x: f64, y: f64) -> MathResult {
            if y == 0.0 {
                Err(MathError::DivisionByZero)
            } else {
                Ok(x / y)
            }
        }
        
        pub fn sqrt(x: f64) -> MathResult {
            if x < 0.0 {
                Err(MathError::NegativeSquareRoot)
            } else {
                Ok(x.sqrt())
            }
        }
        
        pub fn ln(x: f64) -> MathResult {
            if x < 0.0 {
                Err(MathError::NegativeLogarithm)
            } else {
                Ok(x.ln())
            }
        }
    }

    fn check_ops(x: f64, y: f64) -> f64 {
        match checked::div(x, y) {
            Err(why) => panic!("{:?}", why),
            Ok(res_ln) => match checked::ln(res_ln) {
                Err(why) => panic!("{:?}", why),
                Ok(res_sqrt) => match checked::sqrt(res_sqrt) {
                    Err(why) => panic!("{:?}", why),
                    Ok(res_final) => res_final,
                },       
            },
        }  
    }
    
    // test failed ???
    println!("{}", check_ops(1000.0, 10.0));
}

fn panic_test() {
    fn try_division(x: i32, y: i32) -> i32 {
        if y == 0 {
            panic!("division by zero...");
        } else {
            x / y
        }
    }

    let _x = Box::new(0_i32);
    try_division(3, 1);
    println!("this point won't be reached...");
}

fn hash_map_test() {
    {
        // Alternate/custom key/types
        use std::collections::HashMap;
        fn call(number: &str) -> &str{
            match number {
                "798-1364" => "We're sorry, the call cannot be completed as dialed.
            Please hang up and try again.",
                "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
                _ => "Hi! Who is this again?"
            }
        }

        let mut contacts = HashMap::new();
        contacts.insert("Daniel", "798-1364");
        contacts.insert("Ashley", "645-7689");
        contacts.insert("Katie", "435-8291");
        contacts.insert("Robert", "956-1745");

        match contacts.get(&"Daniel") {
            Some(&number) => println!("calling: {}", call(number)),
            _ => println!("don't have Daniel's number..."),
        }
        
        contacts.insert("Daniel", "164-6743");        
        match contacts.get(&"Ashley") {
            Some(&number) => println!("calling: {}", call(number)),
            _ => println!("fuck shit..."),
        }

        contacts.remove(&"Ashley");
        for (&contact, &number) in contacts.iter() {
            println!("calling {}: {}", contact, number);
        }
    }
    {
        //... 任何实现了 `Eq/Hash` trait 的类型，都可以充当 `HashMap` 的键
        //... f32/f64 没有实现 Hash
        //... 对于集合类，例如 T 实现了 Hash，则 Vec<T> 也实现了 Hash
        //... 自定义类型可以轻易实现Eq/Hash，只要加上 #[derive(PartialEq, Eq, Hash)]
        use std::collections::HashMap;

        #[derive(PartialEq, Eq, Hash)]
        struct Account<'a> {
            username: &'a str,
            password: &'a str,
        }

        struct AccountInfo<'a> {
            name: &'a str,
            email: &'a str,
        }

        type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;
        
        fn try_logon<'a>(accounts: &Accounts<'a>,
                    username: &'a str, password: &'a str) {
            println!("username: {}", username);
            println!("password: {}", password);
            println!("Attempting logon...");
            
            let logon = Account {
                username: username,
                password: password,
            };

            match accounts.get(&logon) {
                Some(account_info) => {
                    println!("logon success...");
                    println!("name: {}", account_info.name);
                    println!("email: {}", account_info.email);
                },
                _ => println!("login failed..."),
            }                
        }

        //...main
        let mut accounts: Accounts = HashMap::new();
        let account = Account {
            username: "masker_dad",
            password: "password123",
        };
        let account_info = AccountInfo {
            name: "zq",
            email: "hellozq@kernel.org", 
        };
        
        accounts.insert(account, account_info);

        try_logon(&accounts, "masker_dad", "psasword123");
        try_logon(&accounts, "masker_dad", "password123");
    }
    {
        // HashSet
        /*
            HashSet 的独特之处在于，它保证了不会出现重复的元素。
            这是任何 set 集合类型（set collection）遵循的规定。
            HashSet 只是它的一个实现。（参见：BTreeSet）
         */
        /*
            集合 set 拥 4 种基本操作 (以下调用均返回一个迭代器):
            1) union: 并集，获得两个集合中的所有元素，不含重复值
            2) difference: 差集，获取属于第一集合，而不属于第二集合的所有元素
            3) intersection: 交集，获取同时属于两个集合的所有元素
            4) symmetric_difference: 对称差，获取所有只属于其中一个集合，而不同时属于
                                     两个集合的所有元素
         */
        use std::collections::HashSet;
        
        let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
        let mut b: HashSet<i32> = vec![2, 3, 4].into_iter().collect();
        
        //assert!(a.insert(3));
        assert!(a.insert(4));
        assert!(a.contains(&4));
        //assert!(b.insert(4), "value 4 already in set b...");
        assert!(b.insert(5));
        
        //若一个集合（collection）的元素类型实现了 `Debug`，那么该集合也就实现了 `Debug`
        println!("a: {:?}", a);
        println!("b: {:?}", b);

        //...union/difference/intersection/symmetric_difference
        println!("union: {:?}", a.union(&b).collect::<Vec<&i32>>());
        println!("difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
        println!("intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());
        println!("symmetric_difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
    }    
}

fn rc_test() {
    /*
        每当克隆一个 Rc 时，Rc 的引用计数就会增加 1
        而每当克隆得到的 Rc 退出作用域时，引用计数就会减少 1
        当 Rc 的引用计数变为 0 时，这意味着已经没有所有者，Rc 和值两者都将被删除
        克隆 Rc 从不执行深拷贝。克隆只创建另一个指向包裹值的指针，并增加计数
     */
    use std::rc::Rc;
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");
        let rc_a = Rc::new(rc_examples);
        println!("ref_count of rc_a: {}", Rc::strong_count(&rc_a));
        {
            println!("--- rc_a is cloned to rc_b ---");
            let rc_b = Rc::clone(&rc_a);
            println!("ref_count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("ref_count of rc_a: {}", Rc::strong_count(&rc_a));
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
            println!("length of the value inside rc_a: {}", rc_a.len());
            println!("value of rc_b: {}", rc_b);
            println!("--- rc_b is dropped out of scope ---");
        }
        println!("ref_count of rc_a: {}", Rc::strong_count(&rc_a));
        println!("--- rc_a is dropped out of scope ---");
    }
    // rc_examples 已经移入 rc_a
    // 当 rc_a 被删时, rc_examples 也被一起删除
    //println!("rc_examples: {}", rc_examples);
}

fn arc_test() {
    /*
        当线程之间所有权需要共享时，可以使用 Arc
        这个结构通过 Clone 实现可以为内存堆中的值的位置创建一个引用指针，同时增加引用计数器
        由于它在线程之间共享所有权，因此当指向某个值的最后一个引用指针退出作用域时，该变量将被删除
     */
    use std::sync::Arc;
    use std::thread;

    let apple = Arc::new("the same apple");
    for _ in 0..=10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
}
