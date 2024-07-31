fn main() {
    println!("Hello, world!");
    
    modules_test();
}

mod my_mod {
    //...默认私有可见性
    fn private_func() {
        println!("This is a private func...");
    }

    pub fn public_func() {
        println!("This is a public func...");
    }

    //...同一模块下，项可以访问另一项，即使它是私有的
    pub fn indirect_access() {
        println!("This is a indirect access...");
    }

    //...模块嵌套
    pub mod nested {
        pub fn function() {
            println!("called my_mod::nested::function()...");
        }

        #[allow(dead_code)]
        fn private_func() {
            println!("called my_mod::nested::private_function()...");
        }

        pub(self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested...");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called my_mode::nested::public_function_in_super_mod...");
        }

        pub(in crate::my_mod) fn public_function_in_my_mod() { 
            println!("called my_mode::nested::public_function_in_my_mod...");
            public_function_in_nested();
        }
    }

    pub fn call_public_function_in_my_mod() {
        println!("called my_mod::call_public_function_in_my_mod()...");
        nested::public_function_in_my_mod();
        nested::public_function_in_super_mod();
    }

    //...只在当前 crate 可见
    pub(crate) fn public_function_in_crate() {
        println!("called my_mod::public_function_in_crate...");
    }
    
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called my_mod::private_nested::function()...");
        }
    }
}

fn modules_test() {
    visibility_test();
    struct_visibility_test();
    the_use_declaration_test();
    super_and_self_test();
    file_hierarchy_test();   
}

fn visibility_test() {
    fn function() {
        println!("called function()...");
    }

    function();
    my_mod::public_func();
    
    //...公开项
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_crate();

    //pub(in path) 只能在指定模块中访问
    //my_mod::nested::public_function_in_my_mod();

    //模块私有项不能直接访问，即便模块是公开的
    //my_mod::private_function();
    //my_mod::nested::private_function();
    //my_mod::private_nested::function();
}

fn struct_visibility_test() {
    mod my_mod {
        pub struct OpenBox<T> {
            pub contents: T,
        }

        #[allow(dead_code)]
        pub struct ClosedBox<T> {
            contents: T,
        }

        impl<T> ClosedBox<T> {
            pub fn new(contents: T) -> ClosedBox<T> {
                ClosedBox {
                    contents,
                }
            }
        }
    }

    {
        let open_box = my_mod::OpenBox{ contents: "public information" };
        println!("The open box contains: {}", open_box.contents);

        //let closed_box = my_mod::ClosedBox{ contents: "closed information" };
        let _closed_box = my_mod::ClosedBox::new("closed information");
        //println!("The closed box contains: {}", _closed_box.contents);
    }
}

fn the_use_declaration_test() {
    //将 `deeply::nested::function` 路径绑定到 `other_function`
    use deeply::nested::function as other_function;
    
    fn function() {
        println!("called function()...");
    }

    mod deeply {
        pub mod nested {
            pub fn function() {
                println!("called deeply::nested::function()...");
            }
        }
    }

    {
        other_function();
        println!("Entering Block...");
        {
            // `use` 绑定拥有局部作用域。在这个例子中，`function()`
            // 的遮蔽只存在这个代码块中。
            use deeply::nested::function;
            function();
            println!("Leaving Block...");
        }
        function();
    }
}

fn super_and_self_test() {
    //https://rustwiki.org/zh-CN/rust-by-example/mod/super.html
}

fn file_hierarchy_test() {
    //https://rustwiki.org/zh-CN/rust-by-example/mod/split.html
}
