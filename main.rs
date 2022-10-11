use std::io;

fn main(){
    let _num = 0b11010101;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error occurred");
    println!("Your input: {input}");
}