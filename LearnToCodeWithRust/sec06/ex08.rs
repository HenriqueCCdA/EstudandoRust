fn main() {
    let mut my_stack_value = 2;
    let mut my_integer_reference = &my_stack_value;

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;
}
