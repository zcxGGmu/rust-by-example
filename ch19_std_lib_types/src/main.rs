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
    
}

fn result_test() {
    // ?    
}

fn panic_test() {
    
}

fn hash_map_test() {
    {
        // Alternate/custom key/types



    }
    {
        // HashSet




    }    
}

fn rc_test() {
    
}

fn arc_test() {
    
}
