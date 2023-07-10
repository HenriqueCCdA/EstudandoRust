fn main() {
    let a: i32 = 10;

    {

        println!("O valor de a é {}", a);
        let a:f32 = 20.309;
        println!("O valor de a é {}", a);

    }

    println!("O valor de a é {}", a);

}
