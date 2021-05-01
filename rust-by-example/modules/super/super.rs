fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");

        // calling `self::function()` and calling `function()` directly
        // both give the same result, because they refer to the same function
        self::function();
        function();

        // we can also use `self` to access another module inside `my`:
        self::cool::function();

        // the `super` keyword refers to the parent scope (outside the 
        // `my` module)
        super::function();

        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
