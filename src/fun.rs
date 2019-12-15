struct Circle {
    x:f64,
    y:f64,
    radius:f64
}
impl Circle{
    fn area(&self) ->f64{
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
trait HasArea {
    fn  area(&self) -> f64;
    fn  is_larger(&self,&Self) ->bool;
}

impl HasArea for  Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    fn is_larger(&self,other:&Self)->bool{
        self.area() > other.area()
    }
}
struct Square {
    x:f64,
    y:f64,
    side:f64,
}

impl  HasArea for Square {
     fn area(&self) -> f64 {
         self.side * self.side
     }
     fn is_larger(&self,other:&Self) -> bool {
         self.area()>other.area()
     }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}
