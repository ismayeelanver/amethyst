use std::any::type_name_of_val;
fn main() {
    let i = String::from("hello world");
    println!("{}", type_name_of_val(&i.parse::<i32>().unwrap()) == "i32")?.except("Error");
}