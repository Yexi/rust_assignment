
enum MyEnum {
    TypeA(TypeA),
    TypeB(TypeB),
    TypeC(TypeC),
}

struct TypeA;
struct TypeB;
struct TypeC;

impl TypeA {
    fn method_a(&self) {
        println!("TypeA method called.");
    }
}

impl TypeB {
    fn method_b(&self) {
        println!("TypeB method called.");
    }
}

impl TypeC {
    fn method_c(&self) {
        println!("TypeC method called.");
    }
}

trait MyTrait {
    fn method(&self);
}

struct SA;
struct SB;
struct SC;

impl MyTrait for SA {
    fn method(&self) {
        println!("SA method called.");
    }
}

impl MyTrait for SB {
    fn method(&self) {
        println!("SB method called.");
    }
}

impl MyTrait for SC {
    fn method(&self) {
        println!("SC method called.");
    }
}



fn main() {
    let vec: Vec<MyEnum> = vec![MyEnum::TypeA(TypeA), MyEnum::TypeB(TypeB), MyEnum::TypeC(TypeC)];
    for item in &vec {
        match item {
            MyEnum::TypeA(type_a) => type_a.method_a(),
            MyEnum::TypeB(type_b) => type_b.method_b(),
            MyEnum::TypeC(type_c) => type_c.method_c(),
        }
    }
    let vec: Vec<Box<dyn MyTrait>> = vec![Box::new(SA), Box::new(SB), Box::new(SC)];
    for item in &vec {
        item.method();
    }

    // 两种不同实现方法的区别：
    // 使用枚举保证类型安全，使用trait object更灵活处理不同类型的数据

}


