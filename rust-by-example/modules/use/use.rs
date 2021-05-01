use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    other_function();

    println!("Entering block");
    {
        // this is equivalent to `use deeply::nested::function as function`
        // this `function()` will shadow the outer one
        use crate::deeply::nested::function;

        function();
        println!("Leaving block");
    }

    function();
}
