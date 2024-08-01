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
