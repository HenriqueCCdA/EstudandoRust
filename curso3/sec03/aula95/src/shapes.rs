pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn draw(&self);
}


pub struct Circle {
    pub radius:  f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        4.14159 * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2. * 3.14159 * self.radius
    }

    fn draw(&self) {
        println!("Desenhando um circulo de raio {}: ", self.radius);
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2. * (self.width + self.height)
    }

    fn draw(&self) {
        println!("Desenhando um retangulo de largura: {} e altura: {} ", self.width, self.height);
    }
}
