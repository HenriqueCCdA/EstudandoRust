mod shapes;

use shapes::{Shape, Circle, Rectangle};

fn main() {
    let c = Circle{ radius: 5.0 };
    let r = Rectangle{ width: 10.0, height: 20.0 };

    println!("Area do Circle: {}", c.area());
    println!("Perimetro do Circle: {}", c.perimeter());
    c.draw();

    println!("Area do Rectangle: {}", r.area());
    println!("Perimetro do Rectangle: {}", r.perimeter());
    r.draw();
}
