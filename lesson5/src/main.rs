
macro_rules! my_macro_rules {
    //匹配无参
    () => {
       println!("my_macro_rules no params"); 
    };
    //匹配一个参数
    ($val:expr) => {
        println!("my_macro_rules with one parameter: {}", $val);
    };
    //匹配多个参数的情况
    ($($val:expr),*) => {
        println!("my_macro_rules with muiltiple parameters: {:?}", [$($val), *]);
    };
}

fn main() {
    let message = "hello world";
    let number1 = 666;
    let number2 = 888;

    my_macro_rules!();
    my_macro_rules!(message);
    my_macro_rules!(number1, number2);
    
}
