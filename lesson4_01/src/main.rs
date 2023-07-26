
enum EColor{
    Red,
    Green,
    Blue,
}

trait Object {
    fn method1(&self);
    fn method2(&self);
    fn method3(&self);
}


struct SColor {
    red: bool,
    green: bool,
    blue: bool,
}

impl Object for SColor {
    fn method1(&self) {
        print!("")
    }

    fn method2(&self) {
        todo!()
    }

    fn method3(&self) {
        todo!()
    }
}



fn main() {
    let colors = vec![EColor::Red, EColor::Green, EColor::Blue];
    
    for color in &colors {
        match color {
            EColor::Red => println!("It's red."),
            EColor::Green => println!("It's green."),
            EColor::Blue => println!("It's blue."),
        }
    }
}


