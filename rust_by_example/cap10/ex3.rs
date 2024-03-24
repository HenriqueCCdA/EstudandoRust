use deeply::nested::function as other_function;


fn function() {
    println!("called ``function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::funncton()`");
        }
    }
}

fn main() {
    other_function();

    println!("Entering black");
    {
        use crate::deeply::nested::function;

        function();

        println!("Leaving block");
    }

    function();
}
