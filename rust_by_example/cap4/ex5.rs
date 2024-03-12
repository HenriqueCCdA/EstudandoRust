fn main() {
    let a_binging;

    {
        let x = 2;

        a_binging = x * x;
    }

    println!("a binding: {}", a_binging);

    let another_binding;

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
