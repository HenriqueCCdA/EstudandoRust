fn main() {
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value;
    println!("{}", my_integer_reference);

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;
    println!("{}", my_heap_reference);
}
